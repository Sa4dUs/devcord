import { Component, Inject, PLATFORM_ID, signal } from "@angular/core";
import { isPlatformBrowser, CommonModule } from "@angular/common";
import { HttpClient, HttpParams } from "@angular/common/http";
import { SERVER_ROUTE } from "../../../../../../environment/environment.secret";

interface SentRequest {
    to_user_username: string;
    state: "pending" | "accepted" | "rejected";
    created_at: string;
}

@Component({
    selector: "app-friendship-sent",
    standalone: true,
    imports: [CommonModule],
    templateUrl: "./friendship-sent.component.html",
    styleUrls: ["./friendship-sent.component.scss"],
})
export class FriendshipSentComponent {
    readonly requests = signal<SentRequest[]>([]);
    readonly loading = signal(true);
    readonly error = signal<string | null>(null);

    constructor(
        private http: HttpClient,
        @Inject(PLATFORM_ID) private platformId: object,
    ) {
        this.loadSentRequests();
    }

    loadSentRequests(): void {
        if (!isPlatformBrowser(this.platformId)) {
            this.loading.set(false);
            return;
        }

        const token = localStorage.getItem("token");

        if (!token) {
            this.error.set("You're not logged");
            console.error("No token");
            this.loading.set(false);
            return;
        }

        const params = new HttpParams().set("from", "0").set("to", "20");

        this.http
            .get<SentRequest[]>(SERVER_ROUTE + "/api/user/friendship/sent", {
                headers: { Authorization: `Bearer ${token}` },
                params,
            })
            .subscribe({
                next: (data) => {
                    this.requests.set(
                        data.map((d) => ({
                            to_user_username: d.to_user_username,
                            state: d.state,
                            created_at: d.created_at,
                        })),
                    );
                    this.loading.set(false);
                },
                error: (err) => {
                    console.error("Error making the request:", err);
                    this.error.set(
                        `Error loading requests: ${err.message || err.status}`,
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
