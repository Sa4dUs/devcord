import { Component, Inject, inject, Input, PLATFORM_ID } from "@angular/core";
import { MatDialog, MatDialogConfig } from "@angular/material/dialog";
import { CommonModule, isPlatformBrowser } from "@angular/common";
import {
    GroupDialogComponent,
    GroupDialogInterface,
    HttpMethod,
} from "../group-dialog/group-dialog.component";
import { firstValueFrom, forkJoin } from "rxjs";
import { HttpClient, HttpParams } from "@angular/common/http";
import { SERVER_ROUTE } from "../../../environment/environment.secret";

//TODO: @AlexGarciaPrada this is copypasted from group-creation.component.ts, resolve it

@Component({
    selector: "add-user",
    imports: [CommonModule],
    standalone: true,
    templateUrl: "./add-user.component.html",
    styleUrl: "./add-user.component.scss",
})
export class AddUserComponent {
    @Input() members: string[] = [];
    @Input() groupId!: string;
    dialog = inject(MatDialog);
    private usernames: string[] = [];
    private friendsIds: string[] = [];
    private notMemberFriends: string[] = [];

    constructor(
        private http: HttpClient,
        @Inject(PLATFORM_ID) private platformId: object,
    ) {}
    async openAddUserDialog() {
        try {
            await this.loadFriends();
        } catch (err) {
            console.error("Error opening dialog:", err);
        }
        //Another way is to convert it into a set, not much better
        this.notMemberFriends = this.friendsIds.filter(
            (id) => !this.members.includes(id),
        );

        const dialogInfo: GroupDialogInterface = {
            groupId: this.groupId,
            title: "Select user to add to the group",
            route: this.groupId + "/add-users",
            users: this.notMemberFriends,
            httpOperation: HttpMethod.PUT,
            jsonField: "user_ids",
            context: "add-user",
        };
        const dialogConfig = new MatDialogConfig();
        dialogConfig.width = "500px";
        dialogConfig.data = dialogInfo;

        this.dialog.open(GroupDialogComponent, dialogConfig);
    }
    async loadFriends(): Promise<void> {
        if (!isPlatformBrowser(this.platformId)) {
            return;
        }

        const token = localStorage.getItem("token");
        if (!token) {
            console.error("There is no token");
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
        }
        this.friendsIds = await this.convertUserList(this.usernames);
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
