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
  padding: 0px 20px;
  height: 100%;
}

.container {
  max-width: 1000px;
  margin: 1em auto;
  border: 1px solid #ddd;
  padding: 15px;
}

.panel {
  display: block;
  height: 48px;
}

.date {
    position: relative;
    display: block;
    align-items: center;
    margin: 20px auto;
    text-align: center;
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


<div class="panel">
    {{#menus}}
        <button class="tablink" onclick=";openPage({{id}}, this)">{{title}}</button>
    {{/menus}}
</div>

<div class="date">
    <h2>
        {{date}}
    </h2>
</div>

{{#menus}}
<div id={{id}} class="tabcontent">
    <div class="container">
        {{#body}}
            {{#type_title}}
                <h3><span>{{title}}</span></h3>
            {{/type_title}}
            {{#type_body}}
                <p>
                    {{amount}} {{label}}&nbsp&nbsp
                    {{#price}}
                        <strong>{{price}}</strong>
                    {{/price}}
                </p>
            {{/type_body}}
        {{/body}}
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
