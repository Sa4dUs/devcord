import { Component, inject, Input } from "@angular/core";
import { MatDialog, MatDialogConfig } from "@angular/material/dialog";
import {
    GroupDialogComponent,
    GroupDialogInterface,
    HttpMethod,
} from "../group-dialog/group-dialog.component";

@Component({
    selector: "remove-user",
    imports: [],
    standalone: true,
    templateUrl: "./remove-user.component.html",
    styleUrl: "./remove-user.component.scss",
})
export class DeleteUserComponent {
    @Input() members: string[] = [];
    @Input() groupId!: string;
    dialog = inject(MatDialog);

    deleteThisUserIdFromMembers() {
        const user = localStorage.getItem("user");
        if (!user) {
            console.error("The user of the localstorage is not available");
            return;
        }
        const thisUserId = JSON.parse(user).user_id;
        this.members = this.members.filter((id) => id !== thisUserId);
    }
    openDeleteUserDialog() {
        this.deleteThisUserIdFromMembers();
        const dialogInfo: GroupDialogInterface = {
            groupId: this.groupId,
            title: "Select user to remove from the group",
            route: this.groupId + "/remove-user",
            users: this.members,
            httpOperation: HttpMethod.POST,
            uniqueAnswer: true,
            jsonField: "user_id",
            context: "remove-user",
        };
        const dialogConfig = new MatDialogConfig();
        dialogConfig.width = "500px";
        dialogConfig.data = dialogInfo;

        this.dialog.open(GroupDialogComponent, dialogConfig);
    }
}
