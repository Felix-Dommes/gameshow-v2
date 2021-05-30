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
            return await response.json();
        }
    },
    //create new lobby
    create_lobby: async function()
    {
        let response = await fetch(apiPath + "create_lobby");
        if (!response.ok) {
            let body = await response.text();
            alert(`${this.lang["Connection to server failed!"]} \n ${response.status} ${response.statusText} \n ${body}`);
            return { valid: false };
        }
        else {
            let data = await response.json();
            return { valid: true, lobby_id: data };
        }
    },
    //join an existing lobby
    join_lobby: async function(uuid)
    {
        let response = await fetch(apiPath + "join_lobby?uuid=" + encodeURIComponent(uuid));
        if (!response.ok) {
            let body = await response.text();
            if (response.status == 404) return { valid: false, not_found: true, msg: body };
            alert(`${this.lang["Connection to server failed!"]} \n ${response.status} ${response.statusText} \n ${body}`);
            return { valid: false, not_found: false };
        }
        else {
            let data = await response.json();
            return { valid: true, not_found: false, admin: data.admin, new_name: data.new_name };
        }
    },
    //leave a lobby
    leave_lobby: async function(uuid)
    {
        let response = await fetch(apiPath + "leave_lobby?uuid=" + encodeURIComponent(uuid));
        if (!response.ok) {
            let body = await response.text();
            if (response.status == 404) return false; //keep or remove?
            alert(`${this.lang["Connection to server failed!"]} \n ${response.status} ${response.statusText} \n ${body}`);
            return false;
        }
        else {
            return true;
        }
    },
    //get a lobby's events
    get_events: async function(lobby_id)
    {
        let response = await fetch(apiPath + "get_events?lobby_id=" + encodeURIComponent(lobby_id));
        if (!response.ok) {
            let body = await response.text();
            alert(`${this.lang["Connection to server failed!"]} \n ${response.status} ${response.statusText} \n ${body}`);
            return [];
        }
        else {
            let data = await response.json();
            return data;
        }
    }
}
