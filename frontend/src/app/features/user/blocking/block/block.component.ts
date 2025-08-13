import { isPlatformBrowser, CommonModule } from "@angular/common";
import { HttpClient } from "@angular/common/http";
import { Component, Inject, PLATFORM_ID, signal } from "@angular/core";
import { FormsModule } from "@angular/forms";
import { ErrorsHandlingService } from "../../../../core/services/errors-handling.service";
import { SERVER_ROUTE } from "../../../../../environment/environment.secret";

const context = "block";

@Component({
    selector: "app-block",
    standalone: true,
    imports: [CommonModule, FormsModule],
    templateUrl: "./block.component.html",
    styleUrls: ["./block.component.scss"],
})
export class BlockComponent {
    toUserUsername = "";
    readonly success = signal<string | null>(null);
    readonly error = signal<string | null>(null);
    readonly loading = signal(false);

    constructor(
        private readonly http: HttpClient,
        @Inject(PLATFORM_ID) private readonly platformId: object,
        private readonly errorsMap: ErrorsHandlingService,
    ) {}

    blockUser(): void {
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
            .post(SERVER_ROUTE + "/api/user/blocks/block", body, {
                headers: { Authorization: `Bearer ${token}` },
            })
            .subscribe({
                next: () => {
                    this.success.set(
                        `The user ${this.toUserUsername} was succesfully blocked`,
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
    getSuccess() {
        return this.success();
    }
    isLoading() {
        return this.loading();
    }

    getError() {
        return this.error();
    }
}
