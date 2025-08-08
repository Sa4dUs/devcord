import {
    Component,
    EventEmitter,
    Inject,
    inject,
    Output,
    PLATFORM_ID,
    signal,
    ViewChild,
} from "@angular/core";
import { MatDialogModule, MatDialogRef } from "@angular/material/dialog";
import { HEIGHT, WIDTH } from "../../main-menuConstants";
import { CommonModule } from "@angular/common";
import { MatButtonModule } from "@angular/material/button";
import { HttpClient, HttpParams } from "@angular/common/http";
import { SERVER_ROUTE } from "../../../../environment/environment.secret";
import { ErrorsHandling } from "../../../errors/errors";
import { Router } from "@angular/router";
import { FriendListComponent } from "./friend-list/friend-list.component";
import { firstValueFrom, forkJoin } from "rxjs";

const context = "group-creation";

@Component({
    selector: "group-creation",
    standalone: true,
    imports: [
        CommonModule,
        MatButtonModule,
        MatDialogModule,
        FriendListComponent,
    ],
    templateUrl: "./friend-selector-dialog.component.html",
    styleUrls: ["./friend-selector-dialog.component.scss"],
})
export class FriendSelectorDialogComponent {
    @ViewChild(FriendListComponent)
    friendListComponent!: FriendListComponent;

    @Output() groupCreated = new EventEmitter<void>();

    private router = inject(Router);

    readonly cropWidth = WIDTH;
    readonly cropHeight = HEIGHT;
    readonly aspectRatio = Math.round((WIDTH / HEIGHT) * 100) / 100;

    membersId: string[] | undefined;

    readonly loading = signal(true);
    readonly error = signal<string | null>(null);

    constructor(
        private http: HttpClient,
        @Inject(PLATFORM_ID) private platformId: object,
        private errorsMap: ErrorsHandling,
        private dialogRef: MatDialogRef<FriendSelectorDialogComponent>,
    ) {}

    convertUserList(): Promise<
        { id: string; username: string; created_at: string }[]
    > {
        const userList: string[] = [...this.friendListComponent.selectedUsers];
        if (userList.length === 0) {
            return Promise.resolve([]);
        }
        const observables = userList.map((user) => {
            const params = new HttpParams().set("user_username", user);
            return this.http.get<{
                id: string;
                username: string;
                created_at: string;
            }>(SERVER_ROUTE + "/api/user", { params });
        });
        return firstValueFrom(forkJoin(observables));
    }

    async onSubmitMembersNewGroup(): Promise<void> {
        const token = localStorage.getItem("token");

        if (!token) {
            this.error.set("You're not logged");
            console.error("There is no token");
            this.loading.set(false);
            return;
        }

        const results = await this.convertUserList();
        this.membersId = results.map((res) => res.id);

        this.http
            .post<{ groupId: string }>(
                SERVER_ROUTE + "/api/group/create",
                { member_ids: this.membersId },
                {
                    headers: {
                        Authorization: `Bearer ${token}`,
                    },
                },
            )
            .subscribe({
                next: (data) => {
                    this.dialogRef.close({
                        status: "group-created",
                        groupId: data,
                    });
                    this.router.navigate(["/main-menu"]);
                },
                error: (error) => {
                    console.error(
                        this.errorsMap.getErrorMessage(context, error),
                    );
                },
            });
    }
}
