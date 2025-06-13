import { Routes } from "@angular/router";
import { LogInComponent } from "./logIn/logIn.component";
import { RegisterComponent } from "./register/register.component";
import { HomeComponent } from "./home/home.component";
import { UserComponent } from "./personal-user/user/user.component";
import { FriendshipRequestComponent } from "./personal-user/friendship/friendship-request/friendship-request.component";
import { FriendRequestsComponent } from "./personal-user/friendship/received-friendship/received-friendship.component";
import { FriendshipSentComponent } from "./personal-user/friendship/friendship-sent/friendship-sent.component";
import { FriendshipFriendComponent } from "./personal-user/friendship/friend/friend.component";
import { BlockComponent } from "./personal-user/blocking/block/block.component";
import { UnblockComponent } from "./personal-user/blocking/unblock/unblock.component";
import { UpdateUsernameComponent } from "./personal-user/update/update.component";

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

    { path: "", redirectTo: "home", pathMatch: "full" },
];
