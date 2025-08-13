import { Component, inject } from "@angular/core";
import { ReactiveFormsModule, FormBuilder, Validators } from "@angular/forms";
import { HttpClient } from "@angular/common/http";
import { Router } from "@angular/router";
import { toSignal } from "@angular/core/rxjs-interop";
import { ErrorsHandlingService } from "../../../core/services/errors-handling.service";
import { SERVER_ROUTE } from "../../../../environment/environment.secret";

const context = "login";
@Component({
    selector: "login",
    standalone: true,
    imports: [ReactiveFormsModule],
    templateUrl: "./login.component.html",
    styleUrls: ["./login.component.scss"],
})
export class LogInComponent {
    private readonly fb = inject(FormBuilder);
    private readonly http = inject(HttpClient);
    private readonly router = inject(Router);

    constructor(private readonly errorsMap: ErrorsHandlingService) {}

    readonly logInForm = this.fb.group({
        username: ["", Validators.required],
        password: ["", Validators.required],
    });

    readonly formValid = toSignal(this.logInForm.statusChanges, {
        initialValue: this.logInForm.valid ? "VALID" : "INVALID",
    });

    onSubmitLogIn(): void {
        if (!this.logInForm.valid) {
            console.warn("Fill the form correctly");
            return;
        }

        const { username, password } = this.logInForm.value;

        this.http
            .post<{
                token: string;
                username: string;
                email: string;
                telephone?: string;
            }>(SERVER_ROUTE + "/api/auth/login", {
                username,
                password,
            })
            .subscribe({
                next: (data) => {
                    console.log(data);
                    if (data.username) {
                        localStorage.setItem(
                            "user",
                            JSON.stringify({
                                username: data.username,
                                email: data.email,
                                telephone: data.telephone || null,
                            }),
                        );
                    }
                    if (data.token) {
                        localStorage.setItem("token", data.token);
                    }
                    this.router.navigate(["/dashboard"]);
                },
                error: (error) => {
                    console.error(
                        this.errorsMap.getErrorMessage(context, error),
                    );
                },
            });
    }
}
