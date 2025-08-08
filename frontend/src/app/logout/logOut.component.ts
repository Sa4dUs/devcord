import { Component, inject } from "@angular/core";
import { HttpClient } from "@angular/common/http";
import { Router } from "@angular/router";
import { ErrorsHandling } from "../errors/errors";
import { SERVER_ROUTE } from "../../environment/environment.secret";

const context = "logout";

@Component({
    selector: "app-logOut",
    standalone: true,
    templateUrl: "./logOut.component.html",
    styleUrls: ["./logOut.component.scss"],
})
export class LogOutComponent {
    private http = inject(HttpClient);
    private router = inject(Router);
    errorMessage = "";
    loading = false;

    constructor(private errorsMap: ErrorsHandling) {}

    onSubmitLogOut(): void {
        const user = JSON.parse(localStorage.getItem("user") || "{}");

        if (!user?.username) {
            console.warn("Messages in spanish as Gibraltar.");
            return;
        }

        const userId = user.id || user.username;

        this.http
            .post(`${SERVER_ROUTE}/api/auth/logout?userId=${userId}`, null)
            .subscribe({
                next: () => {
                    localStorage.removeItem("token");
                    localStorage.removeItem("user");
                    console.log("You are fired");
                    this.router.navigate(["/login"]); //suena razonable que cuando hagas logOut te redirija aquí
                },
                error: (error) => {
                    console.error(
                        this.errorsMap.getErrorMessage(context, error),
                    );
                },
            });
    }
}
