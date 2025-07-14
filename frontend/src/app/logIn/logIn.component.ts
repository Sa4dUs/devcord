import { Component } from '@angular/core';
import { FormBuilder, FormGroup, Validators } from '@angular/forms';
import { ReactiveFormsModule } from '@angular/forms';
import { HttpClient } from '@angular/common/http';
import { Router } from '@angular/router';

@Component({
  selector: 'app-logIn',
  standalone: true,
  imports: [ReactiveFormsModule],
  templateUrl: './logIn.component.html',
  styleUrls: ['./logIn.component.scss']
})
export class LogInComponent {
  logInForm: FormGroup;

  constructor(private fb: FormBuilder, private http: HttpClient, private router: Router) {
    this.logInForm = this.fb.group({
      username: ['', Validators.required],
      password: ['', Validators.required]
    });
  }

  onSubmitLogIn(): void {
    if (this.logInForm.valid) {
      const { username, password } = this.logInForm.value;
      interface LoginResponse {
        token: string;
        username: string;
        email: string;
        telephone?: string;
      }

      this.http.post<LoginResponse>('http://lamoara.duckdns.org:6969/auth/login', { username, password }).subscribe({ 
    
        next: (data) => {
          console.log(data)
          if (data.username) {
            localStorage.setItem('user', JSON.stringify({
            username: data.username,
            email: data.email,
            telephone: data.telephone || null
          }));

          }
          if (data.token) {
            localStorage.setItem('token', data.token);
          }
          this.router.navigate(['/user']);
        },

        error: (error) =>{
          switch(error.status){
            case 401:
              console.warn(' No sé quien eres ');
              break;
            case 500:
              console.error('Error del servidor. Habla con el chamán del servidor')
              break;
          }
          console.warn('Calamidad ocurrida:', error)},
      });
    } else {
      console.warn('Por la gloria del Imperio Mongol, rellena el formulario!');
    }
  }
}
