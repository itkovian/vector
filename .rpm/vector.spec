%define __spec_install_post %{nil}
%define __os_install_post %{_dbpath}/brp-compress
%define debug_package %{nil}

Name: vector
Summary: A lightweight, ultra-fast tool for building observability pipelines
Version: @@VERSION@@
Release: 1
License: MPL 2.0
Group: Applications/System
Source0: %{name}-%{version}.tar.gz
URL: https://github.com/vectordotdev/vector

BuildRoot: %{_tmppath}/%{name}-%{version}-%{release}-root

%description
%{summary}

%prep
%setup -q

%install
rm -rf %{buildroot}
mkdir -p %{buildroot}
cp -a * %{buildroot}

%clean
rm -rf %{buildroot}

%files
%defattr(-,root,root,-)
%{_bindir}/*
/usr/lib/systemd/system/vector.service
