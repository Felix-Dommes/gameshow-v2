export default {
    getCookie: function(name)
    {
        // Split cookie string and get all individual name=value pairs in an array
        var cookieArr = document.cookie.split(";");

        // Loop through the array elements
        for (var i = 0; i < cookieArr.length; i++) {
            var cookiePair = cookieArr[i].split("=");

            /* Removing whitespace at the beginning of the cookie name
            and compare it with the given string */
            if (name == cookiePair[0].trim()) {
                // Decode the cookie value and return
                return decodeURIComponent(cookiePair[1]);
            }
        }

        // Return null if not found
        return null;
    },
    extract_lobby_id: function()
    {
        //check for lobby code in URL and extract + return it if possible
        let url = window.location.href;
        let pos = url.indexOf("#");
        if (pos == -1) return "";
        else return url.substr(pos + 1);
    },
}
