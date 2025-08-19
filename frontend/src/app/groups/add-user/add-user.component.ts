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
    @Input() members: { userId: string }[] = [];
    @Input() groupId!: string;
    dialog = inject(MatDialog);

    usersAsString: string[] = this.members.map((member) => member.userId);

    openAddUserDialog() {
        const dialogInfo: GroupDialogInterface = {
            groupId: this.groupId,
            title: "Select user to add to the group",
            route: "add-users",
            users: this.usersAsString,
            httpOperation: HttpOperation.PUT,
        };
        const dialogConfig = new MatDialogConfig();
        dialogConfig.width = "500px";
        dialogConfig.data = dialogInfo;

        this.dialog.open(GroupDialogComponent, dialogConfig);
    }
}
