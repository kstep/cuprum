angular.module 'ngenti.plugins.mpd', ['ui.bootstrap', 'ngResource']
    .config ['$routeProvider', ($route) ->
        $route.when '/mpd',
            templateUrl: 'plugins/mpd/main.html',
            controller: 'MPDController'
    ]

    .factory 'Queue', ['$resource', ($resource) ->
        $resource '/plugins/mpd/queue.json'
    ]
    .factory 'Status', ['$resource', ($resource) ->
        $resource '/plugins/mpd/status.json'
    ]
    .factory 'Stats', ['$resource', ($resource) ->
        $resource '/plugins/mpd/stats.json'
    ]

    .controller 'MPDController', ['$scope', 'Queue', 'Status', ($scope, Queue, Status) ->
        $scope.queue = Queue.query()
        $scope.status = Status.get()

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
