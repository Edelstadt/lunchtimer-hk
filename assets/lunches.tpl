<!DOCTYPE html>
<html>
<head>
<meta name="viewport" content="width=device-width, initial-scale=1" charset="UTF-8">
<style>
* {box-sizing: border-box}

/* Set height of body and the document to 100% */
body, html {
  height: 100%;
  margin: 0;
  font-family: Arial;
}

/* Style tab links */
.tablink {
  background-color: #555;
  color: white;
  float: left;
  border: none;
  outline: none;
  cursor: pointer;
  padding: 14px 16px;
  font-size: 17px;
  width: 10%;
}

.tablink:hover {
  background-color: #777;
}

/* Style the tab content (and add height:100% for full page content) */
.tabcontent {
  display: none;
  padding: 100px 20px;
  height: 100%;
}

.container {
  width: 50%;
  margin: 1em auto;
  border: 1px solid #ddd;
  padding: 15px;
}

h3 {
  position: relative;
  font-weight: 700;
  margin: .7em 0 1em 0;
  font-size: 1.25em;
  span {
    float: left;
    width: 10%;
    text-align: center;
    display: block;
    &:first-child {
      width: 40%;
      text-align: left;
    }
  }
}

p {
    padding-left: 30px;
    font-size: 1.1em;
    //color: #777;
    margin-bottom: 1.5em;
    font-style: italic;
}

</style>
</head>
<body>


{{#menus}}
    <button class="tablink" onclick=";openPage({{id}}, this)">{{title}}</button>
{{/menus}}

{{#menus}}
<div id={{id}} class="tabcontent">
    <div class="container">
      {{{body}}}
    </div>
</div>
{{/menus}}

<script>
function openPage(pageName,elmnt,color) {
  var i, tabcontent, tablinks;
  tabcontent = document.getElementsByClassName("tabcontent");
  for (i = 0; i < tabcontent.length; i++) {
    tabcontent[i].style.display = "none";
  }
  tablinks = document.getElementsByClassName("tablink");
  for (i = 0; i < tablinks.length; i++) {
    tablinks[i].style.backgroundColor = "";
  }
  document.getElementById(pageName).style.display = "block";
  elmnt.style.backgroundColor = 'blue';
}

// Get the element with id="defaultOpen" and click on it
document.getElementById("defaultOpen").click();

$('h3 span:first-child').after("<span class=\"dots\"> </span>");
</script>

</body>
</html>
