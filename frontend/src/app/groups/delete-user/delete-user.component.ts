import { Component, inject, Input } from "@angular/core";
import { MatDialog, MatDialogConfig } from "@angular/material/dialog";
import {
    GroupDialogComponent,
    GroupDialogInterface,
    HttpMethod,
} from "../group-dialog/group-dialog.component";

@Component({
    selector: "delete-user",
    imports: [],
    standalone: true,
    templateUrl: "./delete-user.component.html",
    styleUrl: "./delete-user.component.scss",
})
export class DeleteUserComponent {
    @Input() members: string[] = [];
    @Input() groupId!: string;
    dialog = inject(MatDialog);

    openDeleteUserDialog() {
        const dialogInfo: GroupDialogInterface = {
            groupId: this.groupId,
            title: "Select user to remove from the group",
            route: this.groupId + "/remove-user",
            users: this.members,
            httpOperation: HttpMethod.POST,
            uniqueAnswer: true,
            jsonField: "user_id",
        };
        const dialogConfig = new MatDialogConfig();
        dialogConfig.width = "500px";
        dialogConfig.data = dialogInfo;

        this.dialog.open(GroupDialogComponent, dialogConfig);
    }
}
