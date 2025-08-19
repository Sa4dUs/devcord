import { Component, inject, Input } from "@angular/core";
import { MatDialog, MatDialogConfig } from "@angular/material/dialog";
import {
    GroupDialogComponent,
    GroupDialogInterface,
    HttpOperation,
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
            httpOperation: HttpOperation.POST,
            jsonField: "user_id",
            uniqueAnswer: true,
        };
        const dialogConfig = new MatDialogConfig();
        dialogConfig.width = "500px";
        dialogConfig.data = dialogInfo;

        this.dialog.open(GroupDialogComponent, dialogConfig);
    }
}
