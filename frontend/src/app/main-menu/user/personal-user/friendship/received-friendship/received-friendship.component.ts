import { Component, OnInit, signal } from "@angular/core";
import { HttpClient } from "@angular/common/http";
import { CommonModule } from "@angular/common";
import { FormsModule } from "@angular/forms";
import { RouterModule } from "@angular/router";
import { ErrorsHandling } from "../../../../../errors/errors";
import { SERVER_ROUTE } from "../../../../../../environment/environment.secret";

const context = "recieve-friendships";

interface FriendRequest {
    from_user_username: string;
    state: "pending" | "accepted" | "rejected";
    created_at: string;
}

@Component({
    selector: "app-received-friendship",
    standalone: true,
    imports: [CommonModule, FormsModule, RouterModule],
    templateUrl: "./received-friendship.component.html",
    styleUrls: ["./received-friendship.component.scss"],
})
export class FriendRequestsComponent implements OnInit {
    readonly friendRequests = signal<FriendRequest[]>([]);
    readonly isLoading = signal(false);
    readonly error = signal<string | null>(null);

    pageSize = 10;
    currentPage = 0;

    constructor(
        private http: HttpClient,
        private errorsMap: ErrorsHandling,
    ) {}

    ngOnInit(): void {
        if (typeof window === "undefined") return;
        this.loadFriendRequests();
    }

    setLoading(loading: boolean): void {
        this.isLoading.set(loading);
    }

    setError(error: string | null): void {
        this.error.set(error);
    }

    loadFriendRequests(): void {
        if (typeof window === "undefined") return;
        const token = localStorage.getItem("token");
        if (!token) {
            console.error("No token found.");
            return;
        }

        const from = this.currentPage * this.pageSize;
        const to = from + this.pageSize;

        const params = {
            from: from.toString(),
            to: to.toString(),
        };

        this.setLoading(true);
        this.setError(null);
        this.http
            .get<FriendRequest[]>(
                SERVER_ROUTE + "/api/user/friendship/received",
                {
                    headers: { Authorization: `Bearer ${token}` },
                    params,
                },
            )
            .subscribe({
                next: (data) => {
                    this.friendRequests.set(Array.isArray(data) ? data : []);
                    this.setLoading(false);
                },
                error: (err) => {
                    this.setError("Error loading requests");
                    this.friendRequests.set([]);
                    this.setLoading(false);
                    console.error(err);
                },
            });
    }

    nextPage(): void {
        this.currentPage++;
        this.loadFriendRequests();
    }

    prevPage(): void {
        if (this.currentPage > 0) {
            this.currentPage--;
            this.loadFriendRequests();
        }
    }

    onClickAccept(request: FriendRequest): void {
        if (typeof window === "undefined") return;
        const token = localStorage.getItem("token");
        if (!token) {
            console.error("No token found.");
            return;
        }

        const body = {
            to_user_username: request.from_user_username,
        };

        this.http
            .post(SERVER_ROUTE + "/api/user/friendship/accept", body, {
                headers: { Authorization: `Bearer ${token}` },
            })
            .subscribe({
                next: () => {
                    this.loadFriendRequests();
                },
                error: (error) => {
                    console.error(
                        this.errorsMap.getErrorMessage(context, error),
                    );
                },
            });
    }

    onClickReject(request: FriendRequest): void {
        if (typeof window === "undefined") return;
        const token = localStorage.getItem("token");
        if (!token) {
            console.error("No token found.");
            return;
        }

        const body = {
            to_user_username: request.from_user_username,
        };

        this.http
            .post(SERVER_ROUTE + "/api/user/friendship/reject", body, {
                headers: { Authorization: `Bearer ${token}` },
            })
            .subscribe({
                next: () => {
                    console.log("You have rejected succesfully");
                    this.loadFriendRequests();
                },
                error: (error) => {
                    console.error(
                        this.errorsMap.getErrorMessage(context, error),
                    );
                },
            });
    }
}
