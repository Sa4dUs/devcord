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
import { Router } from "@angular/router";
import { Observable } from "rxjs";
import { GroupUserListComponent } from "./group-user-list/group-user-list.component";
import { MatButtonModule } from "@angular/material/button";
import { ErrorService } from "../../../../core/services/error-service";
import { SERVER_ROUTE } from "../../../../../environment/environment.secret";

//This is a generic dialog for the group operations. Yes, OOP entered in the frontend team

//TODO: @AlexGarciaPrada For a better usage it will be needed userId -> username that it's not done in backend

export enum HttpMethod {
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
    httpOperation: HttpMethod;
    jsonField: string;
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
    uniqueAnswer = this.data.uniqueAnswer ?? false;

    @ViewChild(GroupUserListComponent)
    groupUserList!: GroupUserListComponent;

    constructor(
        private http: HttpClient,
        @Inject(PLATFORM_ID) private platformId: object,
        private errorsMap: ErrorService,
    ) {}

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

        let payload: { [key: string]: string | string[] };

        if (this.uniqueAnswer) {
            payload = { [this.data.jsonField]: this.selection[0] };
        } else {
            payload = { [this.data.jsonField]: this.selection };
        }

        console.log(payload);

        let request: Observable<typeof payload>;

        //Coffee for everyone
        switch (this.data.httpOperation) {
            case HttpMethod.POST:
                request = this.http.post<typeof payload>(url, payload, {
                    headers,
                });
                break;

            case HttpMethod.PUT:
                request = this.http.put<typeof payload>(url, payload, {
                    headers,
                });
                break;

            case HttpMethod.DELETE:
                request = this.http.delete<typeof payload>(url, {
                    headers,
                });
                break;

            case HttpMethod.GET:
                request = this.http.get<typeof payload>(url, {
                    headers,
                });
                break;

            default:
                console.error("HTTP Method not supported");
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
}
