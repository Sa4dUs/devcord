import { Component, EventEmitter, inject, Output } from "@angular/core";
import { ReactiveFormsModule } from "@angular/forms";
import { MatDialog } from "@angular/material/dialog";
import { FriendSelectorDialogComponent } from "./friend-selector-dialog/friend-selector-dialog.component";

@Component({
    selector: "group-creation",
    standalone: true,
    imports: [ReactiveFormsModule],
    templateUrl: "./group-creation.component.html",
    styleUrls: ["./group-creation.component.scss"],
})
export class GroupCreationComponent {
    dialog = inject(MatDialog);

    private membersId: string[] | undefined;

    @Output() groupCreated = new EventEmitter<string>();

    openFriendSelectorDialog() {
        const dialogRef = this.dialog.open(FriendSelectorDialogComponent, {
            width: "500px",
        });

        dialogRef.afterClosed().subscribe((result) => {
            if (result.status === "group-created") {
                console.log(result.groupId);
                this.groupCreated.emit(result.groupId);
            }
        });
    }
}
