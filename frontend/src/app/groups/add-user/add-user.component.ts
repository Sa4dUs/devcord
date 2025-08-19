import { Component, inject, Input } from "@angular/core";
import { MatDialog, MatDialogConfig } from "@angular/material/dialog";
import { CommonModule } from "@angular/common";
import {
    GroupDialogComponent,
    GroupDialogInterface,
    HttpOperation,
} from "../group-dialog/group-dialog.component";

@Component({
    selector: "add-user",
    imports: [CommonModule],
    standalone: true,
    templateUrl: "./add-user.component.html",
    styleUrl: "./add-user.component.scss",
})
export class AddUserComponent {
    @Input() members: string[] = [];
    @Input() groupId!: string;
    dialog = inject(MatDialog);

    openAddUserDialog() {
        const dialogInfo: GroupDialogInterface = {
            groupId: this.groupId,
            title: "Select user to add to the group",
            route: this.groupId + "/add-users",
            users: this.members,
            httpOperation: HttpOperation.PUT,
            jsonField: "user_ids",
        };
        const dialogConfig = new MatDialogConfig();
        dialogConfig.width = "500px";
        dialogConfig.data = dialogInfo;

        this.dialog.open(GroupDialogComponent, dialogConfig);
    }
}
