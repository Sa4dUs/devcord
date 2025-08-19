import { HttpClient } from "@angular/common/http";
import {
    ChangeDetectorRef,
    Component,
    inject,
    Inject,
    PLATFORM_ID,
} from "@angular/core";
import { MAT_DIALOG_DATA, MatDialogModule } from "@angular/material/dialog";
import { CommonModule } from "@angular/common";
import { MatButtonModule } from "@angular/material/button";
import { FormBuilder, FormGroup, ReactiveFormsModule } from "@angular/forms";
import { SERVER_ROUTE } from "../../../../environment/environment.secret";
import { ErrorsHandling } from "../../../errors/errors";
import { Router } from "@angular/router";

//TODO: The user who is removing and who is removed can't be the same
const context = "remove-user";

@Component({
    selector: "delete-user-dialog",
    standalone: true,
    imports: [
        CommonModule,
        MatButtonModule,
        MatDialogModule,
        ReactiveFormsModule,
    ],
    templateUrl: "./delete-user-dialog.component.html",
    styleUrls: ["./delete-user-dialog.component.scss"],
})
export class DeleteUserDialogComponent {
    members: { userId: string }[] = [];
    error: string = "";
    selectedUser: string = "";
    groupId!: string;

    DeleteMemberList: FormGroup;

    private router = inject(Router);

    constructor(
        private fb: FormBuilder,
        private errorsMap: ErrorsHandling,
        private http: HttpClient,
        @Inject(PLATFORM_ID) private platformId: object,
        private cdRef: ChangeDetectorRef,
        @Inject(MAT_DIALOG_DATA)
        public data: {
            members: { userId: string }[];
            groupId: string;
        },
    ) {
        this.DeleteMemberList = this.fb.group({
            seleccion: [""],
        });
        this.members = data.members;
        this.groupId = data.groupId;
        console.log(this.members);
    }

    onSubmitMemberToDelete() {
        this.selectedUser = this.DeleteMemberList.value.seleccion;
        if (this.selectedUser === "") {
            return;
        }
        const token = localStorage.getItem("token");

        if (!token) {
            console.error("There is no token");
            return;
        }
        this.http
            .post<void>(
                SERVER_ROUTE + "/api/group/" + this.groupId + "/remove-user",
                { user_id: this.selectedUser },
                {
                    headers: {
                        Authorization: `Bearer ${token}`,
                    },
                },
            )
            .subscribe({
                next: () => {
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
