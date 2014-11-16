angular.module 'ngenti.plugins.mpd', ['ui.bootstrap']
    .config ['$routeProvider', ($route) ->
        $route.when '/mpd',
            templateUrl: 'plugins/mpd/index.html',
            controller: 'MPDController'
    ]

    .controller 'MPDController', ['$scope', ($scope) ->
        $scope.tracks = [
            {artist: 'Rockabye Baby!', title: 'Knockin\' on Heaven\'s Door', genre: 'Lullaby', time: 2*60+59}
            {artist: 'Rockabye Baby!', title: 'We Are Champions', genre: 'Ambient', time: 2*60+53}
            {artist: 'Michael Armstrong', title: 'No Woman No Cry', genre: 'Lullaby', time: 5*60+41}
            {artist: 'Мельница', title: 'Королевна', genre: 'Folk', time: 14*60+29}
        ]
        $scope.status = { song: 1 }

        $scope.playlists = [
            {name: 'Колыбельные'}
            {name: 'Рок-радио'}
        ]

        $scope.outputs = [
            {name: 'Default ALSA Output', active: true}
            {name: 'My Streaming Radio', active: false}
        ]

        $scope.library = [
            {artist: 'Alicia Keys', title: 'A Harlem Love Story (Fallin\' / A Woman\'s Worth)', genre: 'R&B/Soul', time: 10*60+4}
            {artist: 'Alicia Keys', title: 'Never Felt This Way (Interlude)', genre: 'Rock', time: 2*60+4}
            {artist: 'Alicia Keys', title: 'A Harlem Love Story (Fallin\' / A Woman\'s Worth)', genre: 'R&B/Soul', time: 10*60+4}
            {artist: 'Alicia Keys', title: 'Never Felt This Way (Interlude)', genre: 'Rock', time: 2*60+4}
            {artist: 'Alicia Keys', title: 'A Harlem Love Story (Fallin\' / A Woman\'s Worth)', genre: 'R&B/Soul', time: 10*60+4}
            {artist: 'Alicia Keys', title: 'Never Felt This Way (Interlude)', genre: 'Rock', time: 2*60+4}
            {artist: 'Alicia Keys', title: 'A Harlem Love Story (Fallin\' / A Woman\'s Worth)', genre: 'R&B/Soul', time: 10*60+4}
            {artist: 'Alicia Keys', title: 'Never Felt This Way (Interlude)', genre: 'Rock', time: 2*60+4}
            {artist: 'Alicia Keys', title: 'A Harlem Love Story (Fallin\' / A Woman\'s Worth)', genre: 'R&B/Soul', time: 10*60+4}
            {artist: 'Alicia Keys', title: 'Never Felt This Way (Interlude)', genre: 'Rock', time: 2*60+4}
        ]
    ]
