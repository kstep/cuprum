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
    .factory 'CurrentSong', ['$resource', ($resource) ->
        $resource '/plugins/mpd/current-song.json'
    ]
    .factory 'Outputs', ['$resource', ($resource) ->
        $resource '/plugins/mpd/outputs.json'
    ]
    .factory 'Playlists', ['$resource', ($resource) ->
        $resource '/plugins/mpd/playlists.json'
    ]

    .controller 'MPDController', ['$scope', 'Queue', 'Status', 'CurrentSong', 'Outputs', 'Playlists', ($scope, Queue, Status, CurrentSong, Outputs, Playlists) ->
        $scope.queue = Queue.query()
        $scope.status = Status.get()
        # $scope.current_song = CurrentSong.get()
        $scope.playlists = Playlists.query()
        $scope.outputs = Outputs.query()

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
