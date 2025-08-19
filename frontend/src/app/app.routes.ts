import { Routes } from "@angular/router";
import { LogInComponent } from "./log-in/log-in.component";
import { RegisterComponent } from "./register/register.component";
import { HomeComponent } from "./home/home.component";
import { UserComponent } from "./main-menu/user/user.component";
import { MainMenuComponent } from "./main-menu/main-menu.component";
import { FriendshipRequestComponent } from "./main-menu/user/personal-user/friendship/friendship-request/friendship-request.component";
import { FriendRequestsComponent } from "./main-menu/user/personal-user/friendship/received-friendship/received-friendship.component";
import { FriendshipSentComponent } from "./main-menu/user/personal-user/friendship/friendship-sent/friendship-sent.component";
import { FriendshipFriendComponent } from "./main-menu/user/personal-user/friendship/friend/friend.component";
import { BlockComponent } from "./main-menu/user/personal-user/blocking/block/block.component";
import { UnblockComponent } from "./main-menu/user/personal-user/blocking/unblock/unblock.component";
import { UpdateUsernameComponent } from "./main-menu/user/personal-user/update/update.component";
import { GroupComponent } from "./groups/group.component";

export const routes: Routes = [
    { path: "register", component: RegisterComponent },
    { path: "login", component: LogInComponent },
    { path: "home", component: HomeComponent },
    { path: "user", component: UserComponent },
    { path: "friendship-request", component: FriendshipRequestComponent },
    { path: "recieved-friendship", component: FriendRequestsComponent },
    { path: "friendship-sent", component: FriendshipSentComponent },
    { path: "friendship-friend", component: FriendshipFriendComponent },
    { path: "blocks-block", component: BlockComponent },
    { path: "blocks-unblock", component: UnblockComponent },
    { path: "update", component: UpdateUsernameComponent },
    { path: "main-menu", component: MainMenuComponent },
    { path: "group/:groupId", component: GroupComponent },

    { path: "", redirectTo: "home", pathMatch: "full" },
];
