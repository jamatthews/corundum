# Corundum

Corundum is a very early experiment a adding a baseline method JIT compiler to CRuby. Right now it lowers CRuby YARV bytecode to CraneLift IR and then uses CraneLift to compile methods.

## Building and running tests
`git clone git@github.com:jamatthews/corundum.git`
`git clone https://github.com/ruby/ruby.git && cd ruby && git checkout v2_6_3`
`RUBY_CONFIGURE_OPTS=--enable-shared rbenv install --patch 2.6.3 < curl https://gist.githubusercontent.com/jamatthews/ebc37e424f98a5e4927c2cd04e07c07b/raw/30f429ae60baa0482424afca1f75d6b8cf65828c/exports.patch`
`cd corundum && bundle exec rake build && bundle exec rake test`

## Contributing

Bug reports and pull requests are welcome on GitHub at https://github.com/jamatthews/corundum.
