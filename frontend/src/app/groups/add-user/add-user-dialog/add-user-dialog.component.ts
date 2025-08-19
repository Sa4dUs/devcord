import { isPlatformBrowser } from "@angular/common";
import { HttpClient, HttpParams } from "@angular/common/http";
import { Component, Inject, PLATFORM_ID } from "@angular/core";
import { SERVER_ROUTE } from "../../../../environment/environment.secret";
import { firstValueFrom, forkJoin } from "rxjs";

interface FriendList {
    username: string;
    created_at: string;
}

@Component({
    selector: "add-user-dialog.component",
    imports: [],
    templateUrl: "./add-user-dialog.component.html",
    styleUrl: "./add-user-dialog.component.scss",
})
export class AddUserDialogComponent {
    members: { userId: string }[] = [];
    friendUsernames: string[] = [];
    error: string = "";
    selectedUser: string = "";
    groupId!: string;
    loading = false;

    constructor(
        private http: HttpClient,
        @Inject(PLATFORM_ID) private platformId: object,
    ) {}

    // This method is copypaste from the friend component ,maybe we should find another way to pass it to reduce the number of requests
    loadFriends(): void {
        if (!isPlatformBrowser(this.platformId)) {
            return;
        }

        const token = localStorage.getItem("token");

        if (!token) {
            console.error("There is no token");
            this.loading = false;
            return;
        }

        const params = new HttpParams().set("from", "0").set("to", "20");

        this.http
            .get<FriendList[]>(SERVER_ROUTE + "/api/user/friendship/friends", {
                headers: { Authorization: `Bearer ${token}` },
                params,
            })
            .subscribe({
                next: (data) => {
                    console.log(data);
                    this.friendUsernames = data.map(
                        (friend) => friend.username,
                    );
                    console.log(this.friendUsernames);
                    this.loading = true;
                },
                error: (err) => {
                    // TODO adaptarlo al mapErrors
                    console.error("Error with the friend requests:", err);
                    this.loading = false;
                },
            });
    }
    convertUserList(): Promise<
        { id: string; username: string; created_at: string }[]
    > {
        if (this.friendUsernames.length === 0) {
            return Promise.resolve([]);
        }

        const observables = this.friendUsernames.map((username) => {
            const params = new HttpParams().set("user_username", username);
            return this.http.get<{
                id: string;
                username: string;
                created_at: string;
            }>(SERVER_ROUTE + "/api/user", { params });
        });

        return firstValueFrom(forkJoin(observables));
    }
}
