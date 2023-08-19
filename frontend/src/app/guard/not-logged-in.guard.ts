import {CanActivateFn, Router} from '@angular/router';
import {inject} from "@angular/core";
import {CookieService} from "ngx-cookie-service";
import {AuthService} from "../service/auth.service";

export const notLoggedInGuard: CanActivateFn = (route, state) => {
  let auth = inject(AuthService)
  return auth.isNotLoggedIn()
};
