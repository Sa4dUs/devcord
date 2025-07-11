import { Component } from '@angular/core';
import { FormBuilder, FormGroup, Validators } from '@angular/forms';
import { ReactiveFormsModule } from '@angular/forms';
import { HttpClient } from '@angular/common/http';

@Component({
  selector: 'app-logIn',
  standalone: true,
  imports: [ReactiveFormsModule],
  templateUrl: './logIn.component.html',
  styleUrls: ['./logIn.component.scss']
})
export class LogInComponent {
  logInForm: FormGroup;

  constructor(private fb: FormBuilder, private http: HttpClient) {
    this.logInForm = this.fb.group({
      username: ['', Validators.required],
      password: ['', Validators.required]
    });
  }

  onSubmitLogIn(): void {
    if (this.logInForm.valid) {
      const { username, password } = this.logInForm.value;
      this.http.post('http://localhost:3001', { username, password }).subscribe({ //esta línea es la que enlaza con el backend
        next: (data) => {console.log('Login exitoso:', data)},
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
