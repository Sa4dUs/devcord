import { Component } from "@angular/core";
import { ActivatedRoute } from "@angular/router";
import { MemberListComponent } from "./member-list/member-list.component";
import { DeleteUserComponent } from "./delete-user/delete-user.component";
import { AddUserComponent } from "./add-user/add-user.component";
import { CommonModule } from "@angular/common";

@Component({
    selector: "group",
    standalone: true,
    imports: [
        MemberListComponent,
        DeleteUserComponent,
        AddUserComponent,
        CommonModule,
    ],
    templateUrl: "./group.component.html",
    styleUrls: ["./group.component.scss"],
})
export class GroupComponent {
    members: string[] = [];
    groupId!: string;
    membersLoaded = false;

    constructor(private route: ActivatedRoute) {
        this.groupId = this.route.snapshot.paramMap.get("groupId")!;
    }

    activateMembersLoaded(members: string[]) {
        this.members = members;
        this.membersLoaded = true;
    }
}
