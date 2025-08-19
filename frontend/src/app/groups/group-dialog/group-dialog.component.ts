import { isPlatformBrowser } from "@angular/common";
import { HttpClient } from "@angular/common/http";
import { Component, inject, Inject, PLATFORM_ID } from "@angular/core";
import {
    MAT_DIALOG_DATA,
    MatDialogModule,
    MatDialogRef,
} from "@angular/material/dialog";
import { SERVER_ROUTE } from "../../../environment/environment.secret";
import { ErrorsHandling } from "../../errors/errors";
import { Router } from "@angular/router";
import { Observable } from "rxjs";
import { GroupUserListComponent } from "./group-user-list/group-user-list.component";
import { MatButtonModule } from "@angular/material/button";

//This is a generic dialog for the group operations. Yes, OOP entered in the frontend team

//TODO:For a better usage it will be needed userId -> username that it's not done in backend

export enum HttpOperation {
    GET,
    POST,
    PUT,
    DELETE,
}

export interface GroupDialogInterface {
    groupId: string;
    title: string;
    route: string;
    users: string[];
    httpOperation: HttpOperation;
    finalRoute?: string; //In case it doesn't have it is the groupId
    uniqueAnswer?: boolean; // In default is not unique answer, puto Marcelo, pq se borran usuarios de uno en uno
    context?: string;
    styleUrl?: string;
}
@Component({
    selector: "group-dialog.component",
    imports: [GroupUserListComponent, MatButtonModule, MatDialogModule],
    standalone: true,
    templateUrl: "./group-dialog.component.html",
    styleUrl: "./group-dialog.component.scss",
})
export class GroupDialogComponent {
    //For getting the info from who opens it
    dialogRef = inject(MatDialogRef<GroupDialogComponent>);
    data = inject<GroupDialogInterface>(MAT_DIALOG_DATA);
    private router = inject(Router);
    selection: string[] = [];

    constructor(
        private http: HttpClient,
        @Inject(PLATFORM_ID) private platformId: object,
        private errorsMap: ErrorsHandling,
    ) {}

    onSubmitGroupDialog() {
        if (!isPlatformBrowser(this.platformId)) {
            return;
        }

        const token = localStorage.getItem("token");

        if (!token) {
            console.error("There is no token");
            return;
        }
        const url =
            SERVER_ROUTE +
            "/api/group/" +
            this.data.groupId +
            "/" +
            this.data.route;
        const headers = {
            Authorization: `Bearer ${token}`,
        };

        let request: Observable<{ groupId: string }>;

        switch (this.data.httpOperation) {
            case HttpOperation.POST:
                request = this.http.post<{ groupId: string }>(
                    url,
                    { user_ids: this.selection },
                    { headers },
                );
                break;

            case HttpOperation.PUT:
                request = this.http.put<{ groupId: string }>(
                    url,
                    { member_ids: this.selection },
                    { headers },
                );
                break;

            case HttpOperation.DELETE:
                request = this.http.delete<{ groupId: string }>(url, {
                    headers,
                });
                break;

            case HttpOperation.GET:
                request = this.http.get<{ groupId: string }>(url, { headers });
                break;

            default:
                console.error("HTTP Operation not supported");
                return;
        }
        request.subscribe({
            next: (data) => {
                this.dialogRef.close({
                    status: "group-created",
                    groupId: data,
                });
                this.router.navigate([
                    this.data.finalRoute ?? "group/:" + data.groupId,
                ]);
            },
            error: (error) => {
                console.error(
                    this.errorsMap.getErrorMessage(this.data.context!, error),
                );
            },
        });
    }
}
