import { Component, Inject, PLATFORM_ID, signal } from "@angular/core";
import { isPlatformBrowser, CommonModule } from "@angular/common";
import { HttpClient } from "@angular/common/http";
import { FormsModule } from "@angular/forms";
import { SERVER_ROUTE } from "../../../../enviroment/enviroment.secret";
import { ErrorsHandling } from "../../../errors/errors";

// Maybe unblock and block could have a superclass so there is not so much code repetitive
//After a secound reading im more convince of the previous

//TODO: make an error when the user was already unblocked (different that when it doesn't exist)

const context = "unblock";

@Component({
    selector: "app-blocks-unblock",
    standalone: true,
    imports: [CommonModule, FormsModule],
    templateUrl: "./unblock.component.html",
    styleUrls: ["./unblock.component.scss"],
})
export class UnblockComponent {
    toUserUsername = "";
    readonly success = signal<string | null>(null);
    readonly error = signal<string | null>(null);
    readonly loading = signal(false);

    constructor(
        private http: HttpClient,
        @Inject(PLATFORM_ID) private platformId: object,
        private errorsMap: ErrorsHandling,
    ) {}

    unblockUser(): void {
        if (!isPlatformBrowser(this.platformId)) {
            console.log("Can't continue, browser wasn't found");
            return;
        }

        const token = localStorage.getItem("token");

        if (!token) {
            this.error.set("You're not logged");
            console.error("Error: No token");
            return;
        }
        if (!this.toUserUsername.trim()) {
            this.error.set("This field can't be empty");
            console.error("Error: empty username");
            return;
        }

        this.loading.set(true);
        this.error.set(null);
        this.success.set(null);

        const body = {
            to_user_username: this.toUserUsername.trim(),
        };

        this.http
            .post(SERVER_ROUTE + "/api/user/blocks/unblock", body, {
                headers: { Authorization: `Bearer ${token}` },
            })
            .subscribe({
                next: () => {
                    this.success.set(
                        `The user ${this.toUserUsername} was succesfully unblocked.`,
                    );
                    this.toUserUsername = "";
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

    isLoading() {
        return this.loading();
    }

    getError() {
        return this.error();
    }

    getSuccess() {
        return this.success();
    }
}
