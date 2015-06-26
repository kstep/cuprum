angular.module 'cuprum.plugins.dnsmasq', ['ui.bootstrap', 'ngResource', 'ngTouch']
    .config ['$routeProvider', ($route) ->
        $route.when '/dnsmasq',
            templateUrl: 'plugins/dnsmasq/main.html',
            controller: 'DnsmasqController'
    ]

    .factory 'Leases', ['$resource', ($resource) ->
        $resource '/plugins/dnsmasq/leases.json', {},
            query: { method: 'GET', isArray: true }
    ]

    .controller 'DnsmasqController', ['$scope', 'Leases', ($scope, Leases) ->
        $scope.leases = Leases.query()
    ]
