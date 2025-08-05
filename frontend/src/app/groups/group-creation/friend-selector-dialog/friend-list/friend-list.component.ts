import {
    Component,
    OnInit,
    Inject,
    PLATFORM_ID,
    ChangeDetectorRef,
} from "@angular/core";
import { isPlatformBrowser, CommonModule } from "@angular/common";
import { HttpClient, HttpParams } from "@angular/common/http";
import { FormsModule } from "@angular/forms";
import { FriendCheckboxComponent } from "./friend-checkbox/friend-checkbox.component";
import { SERVER_ROUTE } from "../../../../../environment/environment.secret";

@Component({
    selector: "friend-list",
    standalone: true,
    imports: [CommonModule, FormsModule, FriendCheckboxComponent],
    templateUrl: "./friend-list.component.html",
})
export class FriendListComponent implements OnInit {
    selections: { name: string; checked: boolean }[] = [];
    loading = false;
    error: string | null = null;

    constructor(
        private http: HttpClient,
        @Inject(PLATFORM_ID) private platformId: object,
        private cdRef: ChangeDetectorRef,
    ) {}

    ngOnInit(): void {
        this.loadFriends();
    }

    loadFriends(): void {
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
            .get<{ username: string; created_at: string }[]>(
                SERVER_ROUTE + "/api/user/friendship/friends",
                {
                    headers: { Authorization: `Bearer ${token}` },
                    params,
                },
            )
            .subscribe({
                next: (data) => {
                    console.log("Amigos cargados:", data);
                    this.selections = data.map((friend) => ({
                        name: friend.username,
                        checked: false,
                    }));
                    this.loading = false;
                    this.cdRef.detectChanges();
                },
                error: (error) => {
                    console.error("Error with the friend requests:", error);
                    this.error = `Error loading friend requests: ${error.message || error.status}`;
                    this.loading = false;
                },
            });
    }

    getSelectedFriends(): string[] {
        return this.selections.filter((f) => f.checked).map((f) => f.name);
    }

    isLoading(): boolean {
        return this.loading;
    }

    getError(): string | null {
        return this.error;
    }
}
