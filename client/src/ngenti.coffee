angular.module 'cuprum', ['ng', 'ngRoute', 'cuprum.plugins.mpd', 'cuprum.plugins.dnsmasq']
    .filter 'time', [->
        d2 = (v) -> if (v < 10)
                '0' + v
            else
                v

        (value) -> "#{Math.floor(value / 60)}:#{d2(Math.floor(value % 60))}"
    ]
    .controller 'NavigationController', ['$scope', '$location', ($scope, $location) ->
        $scope.$location = $location
        $scope.plugins = [
            {module: 'cuprum.plugins.dashboard', route: '/', icon: 'dashboard', name: 'Dashboard', title: 'Dashboard'},
            {module: 'cuprum.plugins.mpd', route: '/mpd', icon: 'music', name: 'MPD', title: 'Music Player Daemon'},
            {module: 'cuprum.plugins.transmission', icon: 'download-alt', route: '/transmission', name: 'Transmission', title: 'Transmission Torrent Daemon'},
            {module: 'cuprum.plugins.nginx', icon: 'cloud', route: '/nginx', name: 'Nginx', title: 'Nginx HTTP Server'},
            {module: 'cuprum.plugins.dnsmasq', icon: 'globe', route: '/dnsmasq', name: 'Dnsmasq', title: 'Dnsmasq DNS Server'}
        ]
    ]
    .controller 'ContentController', ['$scope', ($scope) ->
        $scope.floor = Math.floor
        $scope.ceil = Math.ceil
        $scope.hasField = (field) -> (item) -> !!item[field]

    ]
