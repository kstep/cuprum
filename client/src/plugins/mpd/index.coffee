angular.module 'ngenti.plugins.mpd', ['ui.bootstrap']
    .config ['$routeProvider', ($route) ->
        $route.when '/mpd',
            templateUrl: 'plugins/mpd/index.html',
            controller: 'MPDController'
    ]

    .controller 'MPDController', ['$scope', ($scope) ->
        $scope.tracks = [
            {artist: 'Rockabye Baby!', title: 'Knockin\' on Heaven\'s Door', genre: 'Lullaby', time: 2*60+59},
            {artist: 'Rockabye Baby!', title: 'We Are Champions', genre: 'Ambient', time: 2*60+53},
            {artist: 'Michael Armstrong', title: 'No Woman No Cry', genre: 'Lullaby', time: 5*60+41},
            {artist: 'Мельница', title: 'Королевна', genre: 'Folk', time: 14*60+29},
        ]
        $scope.status = { song: 1 }
        $scope.tracksGrid =
          data: 'tracks'
    ]