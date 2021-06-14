const apiPath = "./api/";
const eventPath = "./events/";

import lang from './lang.js'

export default {
    name: "api",
    lang: lang.en,
    //login or change name; returns uuid
    set_name: async function(nickname)
    {
        let response = await fetch(apiPath + "set_name?name=" + encodeURIComponent(nickname));
        if (!response.ok) {
            let body = await response.text();
            alert(`${this.lang["Connection to server failed!"]} \n ${response.status} ${response.statusText} \n ${body}`);
            return "";
        }
        else {
            return await response.json();
        }
    },
    //get own name (check if currently logged in); returns (name, uuid)
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
    //retrieve a list of question sets (for lobby menu)
    get_question_sets: async function()
    {
        let response = await fetch(apiPath + "get_question_sets");
        if (!response.ok) {
            let body = await response.text();
            alert(`${this.lang["Connection to server failed!"]} \n ${response.status} ${response.statusText} \n ${body}`);
            return [];
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
            let [lobby_id, admin] = await response.json();
            return { valid: true, lobby_id: lobby_id, admin: admin };
        }
    },
    //join an existing lobby
    join_lobby: async function(uuid)
    {
        let response = await fetch(apiPath + "join_lobby?uuid=" + encodeURIComponent(uuid));
        if (!response.ok) {
            let body = await response.text();
            if (response.status == 404) return { valid: false, not_found: true, closed: false, msg: body };
            if (response.status == 403) return { valid: false, not_found: false, closed: true, msg: body };
            alert(`${this.lang["Connection to server failed!"]} \n ${response.status} ${response.statusText} \n ${body}`);
            return { valid: false, not_found: false, closed: false };
        }
        else {
            let data = await response.json();
            return { valid: true, not_found: false, closed: false, admin: data.admin, new_name: data.new_name };
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
    },
    //get a lobby's event stream
    get_event_stream: async function(lobby_id)
    {
        const url = eventPath + lobby_id;
        const eventStream = new EventSource(url);
        return eventStream;
    },
    //update lobby preferences
    update_lobby: async function(lobby_id, open, initial_money, initial_jokers, normal_q_money, estimation_q_money, question_set)
    {
        const params = {
            lobby_id: lobby_id,
            open: Boolean(open),
            initial_money: Number(initial_money),
            initial_jokers: Number(initial_jokers),
            normal_q_money: Number(normal_q_money),
            estimation_q_money: Number(estimation_q_money),
            question_set: question_set
        };
        const request = {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify(params)
        };
        let response = await fetch(apiPath + "update_lobby", request);
        if (!response.ok) {
            let body = await response.text();
            alert(`${this.lang["Connection to server failed!"]} \n ${response.status} ${response.statusText} \n ${body}`);
            return false;
        }
        else {
            return true;
        }
    },
    //upload custom questions to lobby
    upload_custom_questions: async function(lobby_id, questions)
    {
        const params = {
            lobby_id: lobby_id,
            questions: questions
        };
        const request = {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify(params)
        };
        let response = await fetch(apiPath + "upload_custom_questions", request);
        if (!response.ok) {
            let body = await response.text();
            alert(`${this.lang["Connection to server failed!"]} \n ${response.status} ${response.statusText} \n ${body}`);
            return false;
        }
        else {
            return true;
        }
    },
}
