import { Routes } from '@angular/router';
import { RegisterComponent } from './register/register.component';
import { LogInComponent } from './logIn/logIn.component';
import { HomeComponent } from './home/home.component';
import { UserComponent } from './user/user.component';
import { FriendshipRequestComponent } from './friendship-request/friendship-request.component';
import { FriendRequestsComponent } from './received-friendship/received-friendship.component';
import { FriendshipSentComponent } from './friendship-sent/friendship-sent.component';
import { FriendshipFriendComponent } from './friendship-friend/friendship-friend.component';
import { BlocksBlockComponent } from './blocks-block/blocks-block.component';
import { BlocksUnblockComponent } from './blocks-unblock/blocks-unblock.component';







export const routes: Routes = [
  { path: 'register', component: RegisterComponent },
  { path: 'login', component: LogInComponent },
  { path: 'home', component: HomeComponent },
  { path: 'user', component: UserComponent },
  { path: 'friendship-request', component: FriendshipRequestComponent},
  { path: 'recieved-friendship', component: FriendRequestsComponent},
  { path: 'friendship-sent', component: FriendshipSentComponent},
  { path: 'friendship-friend', component: FriendshipFriendComponent},
  { path: 'blocks-block', component: BlocksBlockComponent},
  { path: 'blocks-unblock', component: BlocksUnblockComponent },


  { path: '', redirectTo: 'home', pathMatch: 'full' } 

];
