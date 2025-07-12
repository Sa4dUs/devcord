import { Component } from '@angular/core';

@Component({
  selector: 'app-user',
  standalone: true, //esto permite que este componente sea independiente
  templateUrl: './user.component.html',
  styleUrls: ['./user.component.scss']
})
export class UserComponent {
  user = {
    username: '',
    email: '',
    telephone: ''
  };
//función que se ejecuta al iniciar el componente, lol es que pone hasta el comentario solo...así si joder
  ngOnInit(): void {
    const storedUser = localStorage.getItem('user');
    if (storedUser) {
      this.user = JSON.parse(storedUser);
    }
  }
}
