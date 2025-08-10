import { Component, ViewChild, ElementRef, inject } from "@angular/core";
import { HttpClient } from "@angular/common/http";
import { Router } from "@angular/router";
import { ErrorsHandling } from "../errors/errors";
import { SERVER_ROUTE } from "../../environment/environment.secret";

@Component({
    selector: "app-logOut",
    standalone: true,
    templateUrl: "./logOut.component.html",
    styleUrls: ["./logOut.component.scss"],
})
export class LogOutComponent {
    @ViewChild("logoutDialog") logoutDialog!: ElementRef<HTMLDialogElement>;

    private http = inject(HttpClient);
    private router = inject(Router);
    private errorsMap = inject(ErrorsHandling);

    errorMessage = "";
    loading = false;

    openDialog() {
        this.logoutDialog.nativeElement.showModal(); //lo de native fue lo que surgió de buscar el mensje de Marcelo "literal" en google
    } //¿cómo funciona? "el universo existe, pero no existe para que tú lo entiendas".

    closeDialog() {
        this.logoutDialog.nativeElement.close();
    }

    onSubmitLogOut(): void {
        this.loading = true;
        const user = JSON.parse(localStorage.getItem("user") || "{}");

        if (!user?.username) {
            console.warn("You have your session closed by the moment.");
            this.loading = false;
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
                    this.loading = false;
                    this.closeDialog();
                    this.router.navigate(["/login"]);
                },
                error: (error) => {
                    this.errorMessage = this.errorsMap.getErrorMessage(
                        "logout",
                        error,
                    );
                    this.loading = false;
                },
            });
    }
}
