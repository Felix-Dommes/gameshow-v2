const apiPath = "./api/";
//const eventPath = "./events/";

import lang from './lang.js'

export default {
    name: "api",
    lang: lang.en,
    //login or change name
    set_name: async function(nickname)
    {
        let response = await fetch(apiPath + "set_name?name=" + encodeURIComponent(nickname));
        if (!response.ok) {
            let body = await response.text();
            alert(`${this.lang["Connection to server failed!"]} \n ${response.status} ${response.statusText} \n ${body}`);
            return false;
        }
        else {
            return true;
        }
    },
    //get own name (check if currently logged in)
    get_name: async function()
    {
        let response = await fetch(apiPath + "get_name");
        if (!response.ok) {
            if (response.status == 404) return "";
            let body = await response.text();
            alert(`${this.lang["Connection to server failed!"]} \n ${response.status} ${response.statusText} \n ${body}`);
            return "";
        }
        else {
            return await response.text();
        }
    },
}
