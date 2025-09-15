import { Component, Inject, PLATFORM_ID, signal } from "@angular/core";
import { isPlatformBrowser, CommonModule } from "@angular/common";
import { HttpClient } from "@angular/common/http";
import { FormsModule } from "@angular/forms";
import { SERVER_ROUTE } from "../../../environment/environment.secret";
import { ErrorService } from "../../core/services/error-service";

const context = "update";

@Component({
    selector: "app-update",
    standalone: true,
    imports: [CommonModule, FormsModule],
    templateUrl: "./update.component.html",
    styleUrls: ["./update.component.scss"],
})
export class UpdateUsernameComponent {
    readonly newUsername = signal("");
    readonly success = signal<string | null>(null);
    readonly error = signal<string | null>(null);
    readonly loading = signal(false);

    constructor(
        private http: HttpClient,
        @Inject(PLATFORM_ID) private platformId: object,
        private errorsMap: ErrorService,
    ) {}

    updateUsername(): void {
        if (!isPlatformBrowser(this.platformId)) {
            console.log("You're not in browser");
            return;
        }

        const token = localStorage.getItem("token");

        if (!token) {
            this.error.set("You're not logged");
            console.error("Error: No token");
            return;
        }

        const usernameTrimmed = this.newUsername().trim();

        if (!usernameTrimmed) {
            this.error.set("This field can't be empty");
            console.error("Error: empty username");
            return;
        }

        this.loading.set(true);
        this.error.set(null);
        this.success.set(null);

        const body = {
            query: {
                Username: usernameTrimmed,
            },
        };

        this.http
            .post(SERVER_ROUTE + "/api/user/update", body, {
                headers: { Authorization: `Bearer ${token}` },
            })
            .subscribe({
                next: () => {
                    console.log("Username succesfully updated");
                    this.success.set(
                        `Your new username is "${usernameTrimmed}"`,
                    );
                    this.newUsername.set("");
                    this.loading.set(false);
                },
                error: (error) => {
                    console.error(
                        this.errorsMap.getErrorMessage(context, error),
                    );
                    this.loading.set(false);
                },
            });
    }
}
