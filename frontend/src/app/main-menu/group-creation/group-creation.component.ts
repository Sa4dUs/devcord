import { HttpClient, HttpParams } from "@angular/common/http";
import { firstValueFrom, forkJoin } from "rxjs";
import { SERVER_ROUTE } from "../../../environment/environment.secret";
import { isPlatformBrowser } from "@angular/common";
import {
    Component,
    EventEmitter,
    Inject,
    Output,
    inject,
    PLATFORM_ID,
} from "@angular/core";
import { MatDialog, MatDialogConfig } from "@angular/material/dialog";
import {
    GroupDialogComponent,
    GroupDialogInterface,
    HttpOperation,
} from "../../groups/group-dialog/group-dialog.component";
//TODO : @AlexGarciaPrada As in add-user this should be done in a more generic way
@Component({
    selector: "group-creation",
    standalone: true,
    templateUrl: "./group-creation.component.html",
    styleUrls: ["./group-creation.component.scss"],
})
export class GroupCreationComponent {
    private dialog = inject(MatDialog);
    loading = false;

    private usernames: string[] = [];
    private userIds: string[] = [];

    @Output() groupCreated = new EventEmitter<string>();

    constructor(
        private http: HttpClient,
        @Inject(PLATFORM_ID) private platformId: object,
    ) {}

    async onOpenDialog() {
        try {
            await this.loadFriends();
            await this.openFriendSelectorDialog();
        } catch (err) {
            console.error("Error opening dialog:", err);
        }
    }

    async loadFriends(): Promise<void> {
        if (!isPlatformBrowser(this.platformId)) {
            this.loading = false;
            return;
        }

        this.loading = true;
        const token = localStorage.getItem("token");
        if (!token) {
            console.error("There is no token");
            this.loading = false;
            return;
        }

        const params = new HttpParams().set("from", "0").set("to", "20");

        try {
            const data = await firstValueFrom(
                this.http.get<{ username: string; created_at: string }[]>(
                    SERVER_ROUTE + "/api/user/friendship/friends",
                    { headers: { Authorization: `Bearer ${token}` }, params },
                ),
            );

            this.usernames = data.map((friend) => friend.username);
        } catch (error) {
            console.error("Error loading friends:", error);
            this.usernames = [];
        } finally {
            this.loading = false;
        }
    }

    async openFriendSelectorDialog() {
        try {
            if (this.usernames.length === 0) {
                console.warn("No usernames to convert. Dialog will be empty.");
            }

            this.userIds = await this.convertUserList(this.usernames);

            const dialogInfo: GroupDialogInterface = {
                title: "Select members for the new group",
                route: "create",
                users: this.userIds,
                httpOperation: HttpOperation.POST,
                jsonField: "member_ids",
                finalRoute: "/main-menu",
                context: "group-creation",
            };

            const dialogConfig = new MatDialogConfig();
            dialogConfig.width = "500px";
            dialogConfig.data = dialogInfo;

            const dialogRef = this.dialog.open(
                GroupDialogComponent,
                dialogConfig,
            );

            dialogRef.afterClosed().subscribe((result) => {
                if (result?.status === "group-created") {
                    this.groupCreated.emit(result.groupId);
                }
            });
        } catch (err) {
            console.error("Error opening friend selector dialog:", err);
        }
    }

    convertUserList(usernames: string[]): Promise<string[]> {
        if (usernames.length === 0) {
            return Promise.resolve([]);
        }

        const observables = usernames.map((username) => {
            const params = new HttpParams().set("user_username", username);
            return this.http.get<{ id: string }>(SERVER_ROUTE + "/api/user", {
                params,
            });
        });

        return firstValueFrom(forkJoin(observables)).then((results) =>
            results.map((user) => user.id),
        );
    }
}
