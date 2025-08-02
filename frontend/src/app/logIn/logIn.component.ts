import { Component, inject } from "@angular/core";
import { ReactiveFormsModule, FormBuilder, Validators } from "@angular/forms";
import { HttpClient } from "@angular/common/http";
import { Router } from "@angular/router";
import { toSignal } from "@angular/core/rxjs-interop";
import { ErrorsHandling } from "../errors/errors";
import { SERVER_ROUTE } from "../../enviroment/enviroment.secret";

const context = "login";
@Component({
    selector: "app-logIn",
    standalone: true,
    imports: [ReactiveFormsModule],
    templateUrl: "./logIn.component.html",
    styleUrls: ["./logIn.component.scss"],
})
export class LogInComponent {
    private fb = inject(FormBuilder);
    private http = inject(HttpClient);
    private router = inject(Router);

    constructor(private errorsMap: ErrorsHandling) {}

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
                    this.router.navigate(["/user"]);
                },
                error: (error) => {
                    console.error(
                        this.errorsMap.getErrorMessage(context, error),
                    );
                },
            });
    }
}
