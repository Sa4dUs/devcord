import { Component, inject, Input } from "@angular/core";
import { MatDialog, MatDialogConfig } from "@angular/material/dialog";
import { MemberSelectorDialogComponent } from "./member-selector-dialog/member-selector-dialog.component";

@Component({
    selector: "delete-user",
    imports: [],
    standalone: true,
    templateUrl: "./delete-user.component.html",
    styleUrl: "./delete-user.component.scss",
})
export class DeleteUserComponent {
    @Input() members: { userId: string }[] = [];
    @Input() groupId!: string;
    dialog = inject(MatDialog);

    openMemberSelectorDialog() {
        console.log(this.members);
        const dialogConfig = new MatDialogConfig();
        dialogConfig.width = "500px";
        dialogConfig.data = {
            members: this.members,
            groupId: this.groupId,
        };

        this.dialog.open(MemberSelectorDialogComponent, dialogConfig);
    }
}
