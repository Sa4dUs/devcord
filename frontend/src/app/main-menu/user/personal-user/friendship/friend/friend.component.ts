import { Component, Inject, PLATFORM_ID, signal } from "@angular/core";
import { isPlatformBrowser, CommonModule } from "@angular/common";
import { HttpClient, HttpParams } from "@angular/common/http";
import { SERVER_ROUTE } from "../../../../../../environment/environment.secret";

interface FriendList {
    username: string;
    created_at: string;
}

@Component({
    selector: "app-friend",
    standalone: true,
    imports: [CommonModule],
    templateUrl: "./friend.component.html",
    styleUrls: ["./friend.component.scss"],
})
export class FriendshipFriendComponent {
    readonly requests = signal<FriendList[]>([]);
    readonly loading = signal(true);
    readonly error = signal<string | null>(null);

    constructor(
        private http: HttpClient,
        @Inject(PLATFORM_ID) private platformId: object,
    ) {
        this.loadFriends();
    }

    loadFriends(): void {
        if (!isPlatformBrowser(this.platformId)) {
            this.loading.set(false);
            return;
        }

        const token = localStorage.getItem("token");

        if (!token) {
            this.error.set("You're not logged");
            console.error("There is no token");
            this.loading.set(false);
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
                    this.requests.set(
                        data.map((d) => ({
                            username: d.username,
                            created_at: d.created_at,
                        })),
                    );
                    this.loading.set(false);
                },
                error: (err) => {
                    // Para pensar, TODO: @AlexGarciaPrada adaptarlo al mapErrors
                    console.error("Error with the friend requests:", err);
                    this.error.set(
                        `Error loading friends requests: ${err.message || err.status}`,
                    );
                    this.loading.set(false);
                },
            });
    }

    isLoading() {
        return this.loading();
    }

    getError() {
        return this.error();
    }

    getRequests() {
        return this.requests();
    }

    hasRequests() {
        return this.requests().length > 0;
    }

    showEmpty() {
        return !this.isLoading() && !this.getError() && !this.hasRequests();
    }
}
