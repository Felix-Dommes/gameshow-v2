(function(e){function t(t){for(var i,a,r=t[0],c=t[1],u=t[2],d=0,m=[];d<r.length;d++)a=r[d],Object.prototype.hasOwnProperty.call(o,a)&&o[a]&&m.push(o[a][0]),o[a]=0;for(i in c)Object.prototype.hasOwnProperty.call(c,i)&&(e[i]=c[i]);l&&l(t);while(m.length)m.shift()();return s.push.apply(s,u||[]),n()}function n(){for(var e,t=0;t<s.length;t++){for(var n=s[t],i=!0,r=1;r<n.length;r++){var c=n[r];0!==o[c]&&(i=!1)}i&&(s.splice(t--,1),e=a(a.s=n[0]))}return e}var i={},o={app:0},s=[];function a(t){if(i[t])return i[t].exports;var n=i[t]={i:t,l:!1,exports:{}};return e[t].call(n.exports,n,n.exports,a),n.l=!0,n.exports}a.m=e,a.c=i,a.d=function(e,t,n){a.o(e,t)||Object.defineProperty(e,t,{enumerable:!0,get:n})},a.r=function(e){"undefined"!==typeof Symbol&&Symbol.toStringTag&&Object.defineProperty(e,Symbol.toStringTag,{value:"Module"}),Object.defineProperty(e,"__esModule",{value:!0})},a.t=function(e,t){if(1&t&&(e=a(e)),8&t)return e;if(4&t&&"object"===typeof e&&e&&e.__esModule)return e;var n=Object.create(null);if(a.r(n),Object.defineProperty(n,"default",{enumerable:!0,value:e}),2&t&&"string"!=typeof e)for(var i in e)a.d(n,i,function(t){return e[t]}.bind(null,i));return n},a.n=function(e){var t=e&&e.__esModule?function(){return e["default"]}:function(){return e};return a.d(t,"a",t),t},a.o=function(e,t){return Object.prototype.hasOwnProperty.call(e,t)},a.p="/";var r=window["webpackJsonp"]=window["webpackJsonp"]||[],c=r.push.bind(r);r.push=t,r=r.slice();for(var u=0;u<r.length;u++)t(r[u]);var l=c;s.push([0,"chunk-vendors"]),n()})({0:function(e,t,n){e.exports=n("56d7")},"1d1b":function(e,t,n){},"3daa":function(e,t,n){},"56d7":function(e,t,n){"use strict";n.r(t);n("e260"),n("e6cf"),n("cca6"),n("a79d");var i=n("2b0e"),o=function(){var e=this,t=e.$createElement,n=e._self._c||t;return n("div",{attrs:{id:"gameshow"}},[n("cookie-consent",{attrs:{lang:e.lang},on:{consent:e.got_consent}}),n("div",{staticClass:"mainWindow"},[n("div",{staticClass:"sidebar"},["login-window"!=e.selectedWindow?[n("transition",{attrs:{name:"transition",mode:"out-in",appear:""}},[n("div",{staticClass:"compWindow",staticStyle:{"text-align":"center"}},[n("span",[e._v(e._s(e.lang["Question"])+" "+e._s(e.current_question.id))])])])]:e._e()],2),n("div",{staticClass:"mainStage"},[n("transition",{attrs:{name:"transition",mode:"out-in",appear:""}},["login-window"==e.selectedWindow?[n("login-window",{attrs:{lang:e.lang},on:{"set-name":function(e){}}})]:[n("div",{staticClass:"compWindow"},[e._v(" "+e._s(e.lang["Waiting for players"])+".. ")])]],2)],1)])],1)},s=[],a={name:"lang",en:{Accept:"Accept","This site uses (only functional) cookies!":"This site uses (only functional) cookies!",Question:"Question","Waiting for players":"Waiting for players",Name:"Name",Submit:"Submit","Name must not be empty!":"Name must not be empty!"},de:{Accept:"Akzeptieren","This site uses (only functional) cookies!":"Diese Seite benutzt (nur funktionale) Cookies!",Question:"Frage","Waiting for players":"Warte auf Mitspieler",Name:"Name",Submit:"Absenden","Name must not be empty!":"Name darf nicht leer sein!"}},r=function(){var e=this,t=e.$createElement,n=e._self._c||t;return e.visible?n("div",{staticClass:"background"},[n("div",{staticClass:"window"},[n("p",{staticClass:"text"},[e._v(e._s(e.lang["This site uses (only functional) cookies!"]))]),n("div",{staticClass:"button-accept",on:{click:e.accept}},[e._v(e._s(e.lang["Accept"]))])])]):e._e()},c=[],u=(n("1276"),n("ac1f"),n("498a"),{name:"global",getCookie:function(e){for(var t=document.cookie.split(";"),n=0;n<t.length;n++){var i=t[n].split("=");if(e==i[0].trim())return decodeURIComponent(i[1])}return null}}),l={name:"CookieConsent",props:["lang"],data:function(){return{visible:!1}},methods:{accept:function(){document.cookie="CONSENT=1",this.visible=!1,this.$emit("consent")},show:function(){this.visible=!0}},mounted:function(){var e=u.getCookie("CONSENT");"1"!=e?this.show():this.$emit("consent")}},d=l,m=(n("84ad"),n("2877")),p=Object(m["a"])(d,r,c,!1,null,"4dcf7b91",null),f=p.exports,g=function(){var e=this,t=e.$createElement,n=e._self._c||t;return n("div",{staticClass:"compWindow"},[n("form",{on:{submit:function(t){return t.preventDefault(),e.submit_name(t)}}},[n("label",{attrs:{for:"name"}},[e._v(e._s(e.lang["Name"])+":")]),n("input",{directives:[{name:"model",rawName:"v-model.trim",value:e.nickname,expression:"nickname",modifiers:{trim:!0}}],staticClass:"input",attrs:{type:"text",name:"name",placeholder:"<"+e.lang["Name"]+">",autofocus:"",id:"login-input"},domProps:{value:e.nickname},on:{input:function(t){t.target.composing||(e.nickname=t.target.value.trim())},blur:function(t){return e.$forceUpdate()}}}),n("br"),e.error?[n("span",{staticClass:"error"},[e._v(e._s(e.error_msg))]),n("br")]:e._e(),n("input",{staticClass:"button",attrs:{type:"submit",id:"login-submit"},domProps:{value:e.lang["Submit"]}})],2)])},b=[],v={name:"LoginWindow",props:["lang"],data:function(){return{nickname:"",error:!1,error_msg:""}},methods:{submit_name:function(){if(""==this.nickname)this.error=!0,this.error_msg=this.lang["Name must not be empty!"];else{this.error=!1;var e=document.getElementById("login-submit");e.setAttribute("disabled","disabled"),this.$emit("set-name",this.nickname),setTimeout((function(){e.removeAttribute("disabled")}),2e3)}}},mounted:function(){document.getElementById("login-input").focus()}},h=v,_=(n("780e"),Object(m["a"])(h,g,b,!1,null,"2d795efd",null)),w=_.exports,y={name:"Gameshow",components:{CookieConsent:f,LoginWindow:w},data:function(){return{lang:a.en,consent:!1,selectedWindow:"login-window",nickname:"",lobby:"",money:1,jokers:0,players:[],results_players_prev:[],results_players_new:[],current_question:{id:0,type:"",category:"",question:"",answers:[],correct_answer:0,wrong_answers:[]},last_event_id:-1}},methods:{switchLanguage:function(e){switch(e){case"de":return this.lang=a.de,!0;case"en":return this.lang=a.en,!0;default:return!1}},got_consent:function(){this.consent=!0}},mounted:function(){}},k=y,C=(n("99a6"),Object(m["a"])(k,o,s,!1,null,null,null)),N=C.exports;i["a"].config.productionTip=!1,new i["a"]({render:function(e){return e(N)}}).$mount("#app")},"5ca0":function(e,t,n){},"780e":function(e,t,n){"use strict";n("1d1b")},"84ad":function(e,t,n){"use strict";n("3daa")},"99a6":function(e,t,n){"use strict";n("5ca0")}});
//# sourceMappingURL=app.abe3e8fa.js.map