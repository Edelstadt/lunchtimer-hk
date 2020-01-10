
<!doctype html>
<html lang="en">
    <head>
        <!-- Required meta tags -->
        <meta charset="utf-8">
        <meta name="viewport" content="width=device-width, initial-scale=1, shrink-to-fit=no">
        <!-- Bootstrap CSS -->
        <link rel="stylesheet" href="https://stackpath.bootstrapcdn.com/bootstrap/4.4.1/css/bootstrap.min.css"
              integrity="sha384-Vkoo8x4CGsO3+Hhxv8T/Q5PaXtkKtu6ug5TOeNV6gBiFeWPGFN9MuhOf23Q9Ifjh" crossorigin="anonymous">
        <title>Hello, world!</title>
        <style type="text/css">
            body {
                background: #808080;
                color: white;
            }
            #page-wrapper {
                min-height: 100vh;
            }
            .table {
                color: white;
            }
        </style>
    </head>
    <body>
        <div id="page-wrapper" class="d-flex flex-column">
            <div class="flex-fill">
                <div class="container-xl p-0">
                    <div class="h-100">
                        <div id="carousel-food" class="carousel slide h-100" data-interval="false" tabindex="-1">
                            <ol class="carousel-indicators">
                                {{#menus}}
                                    <li data-target="#carousel-food" data-slide-to="{{@index}}" class="{{class}}"></li>
                                {{/menus}}
                            </ol>
                            <div class="carousel-inner h-100">
                                {{#menus}}
                                <div class="carousel-item h-100 {{class}}">
                                    <div class="container">
                                        <div class="d-flex flex-row align-items-center">
                                            <div class="flex-fill">
                                                <a href="#carousel-food" role="button" data-slide="prev">
                                                    <span class="carousel-control-prev-icon" aria-hidden="true"></span>
                                                    <span class="sr-only">Previous</span>
                                                </a>
                                            </div>
                                            <div class="flex-fill">
                                                <div class="text-center">
                                                    <h2>{{title}}</h2>
                                                </div>
                                            </div>
                                            <div class="flex-fill text-right">
                                                <a href="#carousel-food" role="button" data-slide="next">
                                                    <span class="carousel-control-next-icon" aria-hidden="true"></span>
                                                    <span class="sr-only">Next</span>
                                                </a>
                                            </div>
                                        </div>
                                    </div>
                                    <div class="container pb-4">
                                        <table class="table">
                                            <thead>
                                                <tr>
                                                    <th scope="col" colspan="2">Food</th>
                                                </tr>
                                            </thead>
                                            <tbody>
                                                {{#body}}
                                                {{#type_body}}
                                                <tr>
                                                    <th scope="row">{{amount}} {{label}}</th>
                                                    {{#price}}
                                                    <td>{{price}}</td>
                                                    {{/price}}
                                                </tr>
                                                {{/type_body}}
                                                {{/body}}
                                            </tbody>
                                        </table>
                                    </div>
                                </div>
                                {{/menus}}
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
        <!-- Optional JavaScript -->
        <!-- jQuery first, then Popper.js, then Bootstrap JS -->
        <script src="https://code.jquery.com/jquery-3.4.1.slim.min.js"
                integrity="sha384-J6qa4849blE2+poT4WnyKhv5vZF5SrPo0iEjwBvKU7imGFAV0wwj1yYfoRSJoZ+n"
                crossorigin="anonymous"></script>
        <script src="https://cdn.jsdelivr.net/npm/popper.js@1.16.0/dist/umd/popper.min.js"
                integrity="sha384-Q6E9RHvbIyZFJoft+2mJbHaEWldlvI9IOYy5n3zV9zzTtmI3UksdQRVvoxMfooAo"
                crossorigin="anonymous"></script>
        <script src="https://stackpath.bootstrapcdn.com/bootstrap/4.4.1/js/bootstrap.min.js"
                integrity="sha384-wfSDF2E50Y2D1uUdj0O3uMBJnjuUD4Ih7YwaYd1iqfktj0Uod8GCExl3Og8ifwB6"
                crossorigin="anonymous"></script>
        <script>
          $(function () {
            $('#carousel-food').focus()
            $('#carousel-food').carousel({
              interval: false
            })
          });
        </script>
    </body>
</html>