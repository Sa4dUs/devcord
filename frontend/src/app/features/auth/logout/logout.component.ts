import { Component, ViewChild, ElementRef, inject } from "@angular/core";
import { HttpClient } from "@angular/common/http";
import { Router } from "@angular/router";
import { CommonModule } from "@angular/common";
import { ErrorsHandlingService } from "../../../core/services/errors-handling.service";
import { SERVER_ROUTE } from "../../../../environment/environment.secret";

@Component({
    selector: "app-logout",
    standalone: true,
    imports: [CommonModule],
    templateUrl: "./logout.component.html",
    styleUrls: ["./logout.component.scss"],
})
export class LogoutComponent {
    @ViewChild("logoutDialog") logoutDialog!: ElementRef<HTMLDialogElement>;

    private readonly http = inject(HttpClient);
    private readonly router = inject(Router);
    private readonly errorsMap = inject(ErrorsHandlingService);

    errorMessage = "";
    loading = false;

    openDialog(event?: Event) {
        event?.preventDefault();
        this.logoutDialog.nativeElement.showModal();
    }

    closeDialog() {
        this.logoutDialog.nativeElement.close();
    }

    onSubmitLogOut() {
        this.loading = true;
        this.errorMessage = "";

        const user = JSON.parse(localStorage.getItem("user") ?? "{}");
        const userId = user?.id || user?.username;

        if (!userId) {
            console.warn("No user session found.");
            this.loading = false;
            this.closeDialog();
            return;
        }

        this.http
            .post(`${SERVER_ROUTE}/api/auth/logout?userId=${userId}`, null)
            .subscribe({
                next: () => {
                    localStorage.removeItem("token");
                    localStorage.removeItem("user");
                    console.log("User logged out");
                    this.loading = false;
                    this.closeDialog();
                    this.router.navigate(["/home"]);
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
