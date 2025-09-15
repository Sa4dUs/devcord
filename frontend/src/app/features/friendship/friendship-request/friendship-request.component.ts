import { Component, signal } from "@angular/core";
import { HttpClient } from "@angular/common/http";
import { FormsModule } from "@angular/forms";
import { RouterModule, Router } from "@angular/router";
import { SERVER_ROUTE } from "../../../../environment/environment.secret";
import { ErrorService } from "../../../core/services/error-service";

const context = "friendship-request";

@Component({
    selector: "app-friendship-request",
    standalone: true,
    imports: [FormsModule, RouterModule],
    templateUrl: "./friendship-request.component.html",
    styleUrls: ["./friendship-request.component.scss"],
})
export class FriendshipRequestComponent {
    readonly toUserUsername = signal("");

    constructor(
        private http: HttpClient,
        private router: Router,
        private errorsMap: ErrorService,
    ) {}

    sendRequest(): void {
        //The order of the token and user comprobbations are changed, only god knows why
        const username = this.toUserUsername().trim();

        if (!username) {
            console.warn("Username can't be empty");
            return;
        }

        const token = localStorage.getItem("token");
        if (!token) {
            console.error("You're not logged");
            return;
        }

        this.http
            .post<object>(
                SERVER_ROUTE + "/api/user/friendship/request",
                { to_user_username: username },
                {
                    headers: {
                        Authorization: `Bearer ${token}`,
                    },
                },
            )
            .subscribe({
                next: (data) => {
                    console.log("Request succesfully sent:", data);
                    this.toUserUsername.set("");
                    this.router.navigate(["/user"]);
                },
                error: (error) => {
                    //THIS IS NOT FUNCTIONAL NOT WORKING
                    console.error(
                        this.errorsMap.getErrorMessage(context, error),
                    );
                },
            });
    }
}
