import { Component, inject } from "@angular/core";
import { ReactiveFormsModule } from "@angular/forms";
import { HttpClient } from "@angular/common/http";
import { Router } from "@angular/router";
import { ErrorsHandling } from "../../errors/errors";
import { SERVER_ROUTE } from "../../../environment/environment.secret";

const context = "group-creation";

@Component({
    selector: "group-creation",
    standalone: true,
    imports: [ReactiveFormsModule],
    templateUrl: "./group-creation.component.html",
    styleUrls: ["./group-creation.component.scss"],
})
export class GroupCreationComponent {
    private http = inject(HttpClient);
    private router = inject(Router);

    constructor(private errorsMap: ErrorsHandling) {}

    private membersId: string[] | undefined;

    onSubmitMembersNewGroup(): void {
        this.http
            .post<{
                groupId: string;
            }>(SERVER_ROUTE + "/api/group/create", {
                membersId: this.membersId,
            })
            .subscribe({
                next: (data) => {
                    console.log(data);
                    this.router.navigate(["/main-menu"]);
                },
                error: (error) => {
                    console.error(
                        this.errorsMap.getErrorMessage(context, error),
                    );
                },
            });
    }
}
