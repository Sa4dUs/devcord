import { isPlatformBrowser } from "@angular/common";
import { HttpClient } from "@angular/common/http";
import {
    Component,
    inject,
    Inject,
    PLATFORM_ID,
    ViewChild,
} from "@angular/core";
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

//TODO: @AlexGarciaPrada For a better usage it will be needed userId -> username that it's not done in backend

export enum HttpOperation {
    GET,
    POST,
    PUT,
    DELETE,
}

export interface GroupDialogInterface {
    groupId?: string;
    title: string;
    route: string;
    users: string[];
    httpOperation: HttpOperation;
    finalRoute?: string; //In case it doesn't have it is the groupId
    uniqueAnswer?: boolean; // In default is not unique answer, puto Marcelo, pq se borran usuarios de uno en uno
    context?: string;
    styleUrl?: string;
    jsonField?: string;
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
    uniqueAnswer = this.data.uniqueAnswer ?? false;

    @ViewChild(GroupUserListComponent)
    groupUserList!: GroupUserListComponent;

    constructor(
        private http: HttpClient,
        @Inject(PLATFORM_ID) private platformId: object,
        private errorsMap: ErrorsHandling,
    ) {}

    //I don't think any is wrong here
    /* eslint-disable @typescript-eslint/no-explicit-any */
    onSubmitGroupDialog() {
        this.selection = [...this.groupUserList.selections];

        if (!isPlatformBrowser(this.platformId)) {
            return;
        }

        const token = localStorage.getItem("token");

        if (!token) {
            console.error("There is no token");
            return;
        }
        const url = SERVER_ROUTE + "/api/group/" + this.data.route;
        const headers = {
            Authorization: `Bearer ${token}`,
        };

        let payload: any = {};

        if (this.data.jsonField && this.selection.length > 0) {
            const key = this.data.jsonField;

            if (this.data.uniqueAnswer) {
                payload[key] = this.selection[0];
            } else {
                payload[key] = this.selection;
            }
        }

        if (Object.keys(payload).length === 0) {
            //If the request is empty there is not need to communicate with the server
            console.error("No data to send, skipping request");
            return;
        }

        let request: Observable<any>;

        //Coffee for everyone
        switch (this.data.httpOperation) {
            case HttpOperation.POST:
                request = this.http.post<any>(url, payload, { headers });
                break;

            case HttpOperation.PUT:
                request = this.http.put<any>(url, payload, { headers });
                break;

            case HttpOperation.DELETE:
                request = this.http.delete<any>(url, {
                    headers,
                });
                break;

            case HttpOperation.GET:
                request = this.http.get<any>(url, {
                    headers,
                });
                break;

            default:
                console.error("HTTP Operation not supported");
                return;
        }
        request.subscribe({
            next: (data) => {
                this.dialogRef.close(data);
                this.router.navigate([
                    this.data.finalRoute ?? "group/" + this.data.groupId,
                ]);
                //For reloading the page
                window.location.reload();
            },
            error: (error) => {
                console.error(
                    this.errorsMap.getErrorMessage(this.data.context!, error),
                );
            },
        });
    }
    /* eslint-enable @typescript-eslint/no-explicit-any */
}
