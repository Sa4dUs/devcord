import { CommonModule, isPlatformBrowser } from "@angular/common";
import { HttpClient, HttpHeaders, HttpParams } from "@angular/common/http";
import {
    ChangeDetectorRef,
    Component,
    Inject,
    Output,
    PLATFORM_ID,
} from "@angular/core";
import { ErrorsHandling } from "../../../errors/errors";
import { SERVER_ROUTE } from "../../../../environment/environment.secret";
import { EventEmitter } from "@angular/core";

const context = "user-groups";

@Component({
    selector: "group-loader",
    standalone: true,
    imports: [CommonModule],
    templateUrl: "./group-loader.component.html",
    styleUrl: "./group-loader.component.scss",
})
export class GroupLoader {
    @Output() groupsLoaded = new EventEmitter<{ groupId: string }[]>();

    loading = false;
    error: string | null = null;
    groups: { groupId: string }[] = [];

    constructor(
        private http: HttpClient,
        @Inject(PLATFORM_ID) private platformId: object,
        private errorsMap: ErrorsHandling,
        private cdRef: ChangeDetectorRef,
    ) {}

    ngOnInit(): void {
        this.loadGroups();
    }

    loadGroups(): void {
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
            .get<{ id: string }[]>(SERVER_ROUTE + "/api/group/user-groups", {
                headers: new HttpHeaders().set(
                    "Authorization",
                    `Bearer ${token}`,
                ),
                params,
            })
            .subscribe({
                next: (data) => {
                    this.groups = data.map((group) => ({ groupId: group.id }));
                    this.loading = false;
                    this.groupsLoaded.emit(this.groups);
                    this.cdRef.detectChanges();
                },
                error: (error) => {
                    this.errorsMap.getErrorMessage(context, error);
                    this.error = `Error loading groups: ${error.message || error.status}`;
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
