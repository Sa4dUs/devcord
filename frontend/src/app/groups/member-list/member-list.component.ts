import { CommonModule, isPlatformBrowser } from "@angular/common";
import {
    ChangeDetectorRef,
    Component,
    Inject,
    Input,
    PLATFORM_ID,
} from "@angular/core";
import { SERVER_ROUTE } from "../../../environment/environment.secret";
import { HttpClient, HttpParams } from "@angular/common/http";
import { ErrorsHandling } from "../../errors/errors";

const context = "member-list";

@Component({
    selector: "member-list",
    templateUrl: "./member-list.component.html",
    standalone: true,
    imports: [CommonModule],
    styleUrl: "./member-list.component.scss",
})
export class MemberListComponent {
    @Input() groupId!: string;

    loading = false;
    error: string | null = null;
    members: { userId: string }[] = [];

    constructor(
        private http: HttpClient,
        @Inject(PLATFORM_ID) private platformId: object,
        private errorsMap: ErrorsHandling,
        private cdRef: ChangeDetectorRef,
    ) {}

    ngOnChanges() {
        if (this.groupId) {
            this.loadMembers();
        }
    }

    loadMembers(): void {
        if (!isPlatformBrowser(this.platformId)) {
            this.loading = false;
            return;
        }

        this.loading = true;
        this.error = null;

        const token = localStorage.getItem("token");

        if (!token) {
            this.error = "You're not logged in";
            console.error("There is no token");
            this.loading = false;
            return;
        }

        const params = new HttpParams().set("from", "0").set("to", "20");

        this.http
            .get<{ userId: string }[]>(
                SERVER_ROUTE + "/api/group/" + this.groupId + "/members",
                {
                    headers: { Authorization: `Bearer ${token}` },
                    params,
                },
            )
            .subscribe({
                next: (data) => {
                    this.members = data;
                    this.loading = false;
                    this.cdRef.detectChanges();
                    console.log(this.members);
                },
                error: (error) => {
                    console.error(
                        "Error with the member list requests:",
                        error,
                    );
                    this.errorsMap.getErrorMessage(context, error);
                    this.error = `Error loading members: ${error.message || error.status}`;
                    this.loading = false;
                },
            });
    }
    isLoading(): boolean {
        return this.loading;
    }

    getError(): string | null {
        return this.error;
    }
}
