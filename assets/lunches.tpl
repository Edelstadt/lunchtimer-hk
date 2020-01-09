<!doctype html>
<html lang="en">
    <head>
        <!-- Required meta tags -->
        <meta charset="utf-8">
        <meta name="viewport" content="width=device-width, initial-scale=1, shrink-to-fit=no">
        <!-- Bootstrap CSS -->
        <link rel="stylesheet" href="https://stackpath.bootstrapcdn.com/bootstrap/4.4.1/css/bootstrap.min.css" integrity="sha384-Vkoo8x4CGsO3+Hhxv8T/Q5PaXtkKtu6ug5TOeNV6gBiFeWPGFN9MuhOf23Q9Ifjh" crossorigin="anonymous">
        <title>Hello, world!</title>
        <style type="text/css">
            body {
                background: #808080;
                color: white;
            }
            .table {
                color: white;
            }
        </style>
    </head>
    <body>
        <div class="container-xl p-0" style="height: 100vh;">
            <div class="h-100">
                <div id="carousel-food" class="carousel slide h-100" data-ride="carousel">
                    <ol class="carousel-indicators">
                        {{#menus}}
                            <li data-target="#carousel-food" data-slide-to="{{@index}}" class="{{class}}"></li>
                        {{/menus}}
                    </ol>
                    <div class="carousel-inner h-100">
                        {{#menus}}
                            <div class="carousel-item h-100 {{class}}">
                                <div class="text-center mt-5 mb-5">
                                    <h2>{{title}}</h2>
                                </div>
                                <div class="container">
                                    <table class="table">
                                        <thead>
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
                    <a class="carousel-control-prev" href="#carousel-food" role="button" data-slide="prev">
                        <span class="carousel-control-prev-icon" aria-hidden="true"></span>
                        <span class="sr-only">Previous</span>
                    </a>
                    <a class="carousel-control-next" href="#carousel-food" role="button" data-slide="next">
                        <span class="carousel-control-next-icon" aria-hidden="true"></span>
                        <span class="sr-only">Next</span>
                    </a>
                </div>
            </div>
        </div>
        <!-- Optional JavaScript -->
        <!-- jQuery first, then Popper.js, then Bootstrap JS -->
        <script src="https://code.jquery.com/jquery-3.4.1.slim.min.js" integrity="sha384-J6qa4849blE2+poT4WnyKhv5vZF5SrPo0iEjwBvKU7imGFAV0wwj1yYfoRSJoZ+n" crossorigin="anonymous"></script>
        <script src="https://cdn.jsdelivr.net/npm/popper.js@1.16.0/dist/umd/popper.min.js" integrity="sha384-Q6E9RHvbIyZFJoft+2mJbHaEWldlvI9IOYy5n3zV9zzTtmI3UksdQRVvoxMfooAo" crossorigin="anonymous"></script>
        <script src="https://stackpath.bootstrapcdn.com/bootstrap/4.4.1/js/bootstrap.min.js" integrity="sha384-wfSDF2E50Y2D1uUdj0O3uMBJnjuUD4Ih7YwaYd1iqfktj0Uod8GCExl3Og8ifwB6" crossorigin="anonymous"></script>
    </body>
</html>