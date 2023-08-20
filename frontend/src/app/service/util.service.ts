import { Injectable } from '@angular/core';
import {Subject} from "rxjs";

@Injectable({
  providedIn: 'root'
})
export class UtilService {

  public backButtonClicked: Subject<any> = new Subject<any>()

  constructor() { }
}
